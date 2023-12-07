use std::sync::mpsc;

use icrate::{
    block2::ConcreteBlock,
    ns_string,
    objc2::ClassType,
    CallKit::{CXCallController, CXHandle, CXHandleTypePhoneNumber, CXStartCallAction},
    Foundation::{NSError, NSUUID},
};

embed_plist::embed_info_plist!("Info.plist");

fn main() {
    let (tx, rx) = mpsc::channel();
    unsafe {
        let ctl = CXCallController::new();
        let uuid = NSUUID::new();
        let handle = CXHandle::initWithType_value(
            CXHandle::alloc(),
            CXHandleTypePhoneNumber,
            ns_string!("8002752273"),
        );
        let action = CXStartCallAction::initWithCallUUID_handle(
            CXStartCallAction::alloc(),
            uuid.as_ref(),
            handle.as_ref(),
        );
        let completion = ConcreteBlock::new(move |err: *mut NSError| {
            println!("comp {:?}", err);
            if !err.is_null() {
                println!("{}", (*err).localizedDescription());
            }
            tx.send(()).unwrap();
        })
        .copy();
        ctl.requestTransactionWithAction_completion(action.as_ref(), &completion);
    }
    println!("waiting");
    rx.recv().unwrap();
}
