use messages::{mailbox::Mailbox, request::Request};
use tokio::runtime::Builder;

#[test]
fn message_box() {
    let mut basic_rt = Builder::new().basic_scheduler().build().unwrap();

    basic_rt.block_on(async {
        let handler = |_mailbox: &mut Mailbox<Request<i32, i32>>, request: Request<i32, i32>| async move {  
            let response = *request.inner() + 1;

            request.respond(response).await.unwrap();
        };

        let mailbox: Mailbox<Request<i32, i32>> = Mailbox::new();

        let mut address = mailbox.address();

        tokio::spawn(mailbox.run_with(handler));

        let (request, response) = Request::new(10);
        address.send(request).await.unwrap();

        let response = response.await.unwrap();
        assert_eq!(response, 11);
    });
}
