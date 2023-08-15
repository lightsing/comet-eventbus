use ce_macros::service;

#[service(MyService2, "topic")]
async fn my_service2(arg0: u8, arg1: String, arg2: Vec<u8>) {
    format!("{}, {}, {:?}", arg0, arg1, arg2);
}