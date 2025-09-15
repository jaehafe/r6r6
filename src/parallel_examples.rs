use futures::{stream, StreamExt, join, try_join};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// 예제 1: 기본적인 병렬 실행 (join!)
pub async fn example_basic_parallel() {
    println!("=== 예제 1: 기본 병렬 실행 ===");
    let start = Instant::now();

    let (result1, result2, result3) = join!(
        async {
            sleep(Duration::from_millis(100)).await;
            "Task 1 완료"
        },
        async {
            sleep(Duration::from_millis(200)).await;
            "Task 2 완료"
        },
        async {
            sleep(Duration::from_millis(150)).await;
            "Task 3 완료"
        }
    );

    println!("결과: {}, {}, {}", result1, result2, result3);
    println!("소요 시간: {:?}\n", start.elapsed());
}

/// 예제 2: 제한된 동시성으로 대량 작업 처리 (buffer_unordered)
pub async fn example_limited_concurrency() {
    println!("=== 예제 2: 제한된 동시성 (최대 3개 동시) ===");
    let start = Instant::now();

    let tasks = (1i32..=10).map(|i| async move {
        let delay = 100 + (i % 3) * 50; // 다양한 지연 시간
        sleep(Duration::from_millis(delay as u64)).await;
        println!("  Task {} 완료 ({}ms)", i, delay);
        i
    });

    let results: Vec<i32> = stream::iter(tasks)
        .buffer_unordered(3) // 최대 3개만 동시 실행
        .collect()
        .await;

    println!("모든 결과: {:?}", results);
    println!("소요 시간: {:?}\n", start.elapsed());
}

/// 예제 3: HTTP 요청 병렬 처리
pub async fn example_http_parallel() {
    println!("=== 예제 3: HTTP 요청 병렬 처리 ===");
    let start = Instant::now();

    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
        "https://httpbin.org/json",
        "https://httpbin.org/user-agent",
        "https://httpbin.org/headers",
    ];

    let client = reqwest::Client::new();
    let requests = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let result = client.get(url).send().await;
            match result {
                Ok(response) => format!("{}: {}", url, response.status()),
                Err(e) => format!("{}: Error - {}", url, e),
            }
        }
    });

    let results: Vec<String> = stream::iter(requests)
        .buffer_unordered(3) // 동시 3개 요청
        .collect()
        .await;

    for result in results {
        println!("  {}", result);
    }
    println!("소요 시간: {:?}\n", start.elapsed());
}

/// 예제 4: 첫 번째 완료 작업 선택 (select!)
pub async fn example_first_complete() {
    println!("=== 예제 4: 첫 번째 완료 작업 선택 ===");

    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        "빠른 작업"
    };

    let task2 = async {
        sleep(Duration::from_millis(300)).await;
        "느린 작업"
    };

    let task3 = async {
        sleep(Duration::from_millis(200)).await;
        "보통 작업"
    };

    // 단순히 첫 번째 완료되는 것을 기다림
    let result = futures::future::select_all(vec![
        Box::pin(task1) as std::pin::Pin<Box<dyn std::future::Future<Output = &str> + Send>>,
        Box::pin(task2) as std::pin::Pin<Box<dyn std::future::Future<Output = &str> + Send>>,
        Box::pin(task3) as std::pin::Pin<Box<dyn std::future::Future<Output = &str> + Send>>,
    ]).await;

    println!("첫 번째 완료: {}", result.0);
    println!();
}

/// 예제 5: 에러 처리가 있는 병렬 실행 (try_join!)
pub async fn example_error_handling() {
    println!("=== 예제 5: 에러 처리가 있는 병렬 실행 ===");

    async fn maybe_fail(id: i32, should_fail: bool) -> Result<String, String> {
        sleep(Duration::from_millis(100)).await;
        if should_fail {
            Err(format!("Task {} 실패", id))
        } else {
            Ok(format!("Task {} 성공", id))
        }
    }

    // 모든 작업이 성공하는 경우
    println!("모두 성공하는 경우:");
    match try_join!(
        maybe_fail(1, false),
        maybe_fail(2, false),
        maybe_fail(3, false)
    ) {
        Ok((r1, r2, r3)) => println!("  성공: {}, {}, {}", r1, r2, r3),
        Err(e) => println!("  실패: {}", e),
    }

    // 하나가 실패하는 경우
    println!("하나가 실패하는 경우:");
    match try_join!(
        maybe_fail(1, false),
        maybe_fail(2, true), // 이것이 실패
        maybe_fail(3, false)
    ) {
        Ok((r1, r2, r3)) => println!("  성공: {}, {}, {}", r1, r2, r3),
        Err(e) => println!("  실패: {}", e),
    }
    println!();
}

/// 예제 6: Stream 변환과 필터링
pub async fn example_stream_processing() {
    println!("=== 예제 6: Stream 변환과 필터링 ===");

    let numbers = stream::iter(1..=20);

    let results: Vec<i32> = numbers
        .map(|x| async move {
            // 비동기 처리 시뮬레이션
            sleep(Duration::from_millis(10)).await;
            x * x
        })
        .buffer_unordered(5) // 5개씩 병렬 처리
        .filter(|&x| async move { x % 2 == 0 }) // 짝수만 필터링
        .collect()
        .await;

    println!("1-20의 제곱 중 짝수: {:?}", results);
    println!();
}

/// 예제 7: 청킹으로 배치 처리
pub async fn example_batch_processing() {
    println!("=== 예제 7: 청킹으로 배치 처리 ===");

    let data = stream::iter(1..=15);

    let batches: Vec<Vec<i32>> = data
        .chunks(4) // 4개씩 묶어서
        .enumerate()
        .map(|(batch_num, chunk)| async move {
            println!("  배치 {} 처리 중: {:?}", batch_num + 1, chunk);
            sleep(Duration::from_millis(100)).await;
            chunk
        })
        .buffer_unordered(2) // 2개 배치 동시 처리
        .collect()
        .await;

    println!("처리된 배치들: {:?}", batches);
    println!();
}

/// 예제 8: fold를 사용한 축적 처리
pub async fn example_fold_accumulation() {
    println!("=== 예제 8: fold를 사용한 축적 처리 ===");

    let stream = stream::iter(1..=10);

    let sum = stream
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("  처리 중: {}", x);
            x
        })
        .buffer_unordered(3)
        .fold(0, |acc, x| async move { acc + x })
        .await;

    println!("총합: {}", sum);
    println!();
}

/// 예제 9: 복잡한 데이터 처리 파이프라인
pub async fn example_complex_pipeline() {
    println!("=== 예제 9: 복잡한 데이터 처리 파이프라인 ===");

    #[derive(Debug)]
    struct User {
        id: i32,
        name: String,
        score: i32,
    }

    // 가상의 사용자 데이터
    let users = vec![
        User { id: 1, name: "Alice".to_string(), score: 85 },
        User { id: 2, name: "Bob".to_string(), score: 92 },
        User { id: 3, name: "Charlie".to_string(), score: 78 },
        User { id: 4, name: "Diana".to_string(), score: 96 },
        User { id: 5, name: "Eve".to_string(), score: 88 },
    ];

    let processed: Vec<String> = stream::iter(users)
        .map(|user| async move {
            // 비동기 점수 처리 시뮬레이션
            sleep(Duration::from_millis(100)).await;
            println!("  {} 처리 중 (점수: {})", user.name, user.score);
            user
        })
        .buffer_unordered(3)
        .filter(|user| {
            let score = user.score;
            async move { score >= 85 }
        }) // 85점 이상만
        .map(|user| async move {
            // 등급 계산
            let grade = if user.score >= 90 { "A" } else { "B" };
            format!("{}: {} ({}점)", user.name, grade, user.score)
        })
        .buffer_unordered(3)
        .collect()
        .await;

    println!("우수 학생들: {:?}", processed);
    println!();
}

/// 예제 10: 타임아웃이 있는 병렬 처리
pub async fn example_with_timeout() {
    println!("=== 예제 10: 타임아웃이 있는 병렬 처리 ===");

    let tasks = (1..=5).map(|i| {
        async move {
            let delay = i * 100; // 점점 더 오래 걸리는 작업
            sleep(Duration::from_millis(delay)).await;
            format!("Task {} 완료 ({}ms)", i, delay)
        }
    });

    // 300ms 타임아웃으로 처리
    let timeout_duration = Duration::from_millis(300);

    match tokio::time::timeout(
        timeout_duration,
        stream::iter(tasks).buffer_unordered(3).collect::<Vec<_>>()
    ).await {
        Ok(results) => {
            println!("시간 내 완료:");
            for result in results {
                println!("  {}", result);
            }
        },
        Err(_) => println!("타임아웃 발생! ({}ms)", timeout_duration.as_millis()),
    }
}