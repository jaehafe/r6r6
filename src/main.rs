use r6r6::parallel_examples::*;

#[tokio::main]
async fn main() {

    for i in 0..100 {
        tokio::spawn(async move {
            println!("Hello from task {}", i);
        });
    }

    println!("🚀 Futures 병렬 처리 예제들\n");

    // 예제 1: 기본 병렬 실행
    example_basic_parallel().await;

    // 예제 2: 제한된 동시성
    example_limited_concurrency().await;

    // 예제 3: HTTP 요청 병렬 처리
    example_http_parallel().await;

    // 예제 4: 첫 번째 완료 작업 선택
    example_first_complete().await;

    // 예제 5: 에러 처리가 있는 병렬 실행
    example_error_handling().await;

    // 예제 6: Stream 변환과 필터링
    example_stream_processing().await;

    // 예제 7: 청킹으로 배치 처리
    example_batch_processing().await;

    // 예제 8: fold를 사용한 축적 처리
    example_fold_accumulation().await;

    // 예제 9: 복잡한 데이터 처리 파이프라인
    example_complex_pipeline().await;

    // 예제 10: 타임아웃이 있는 병렬 처리
    example_with_timeout().await;

    println!("✅ 모든 예제 완료!");
}