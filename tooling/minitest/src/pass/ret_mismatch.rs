use crate::*;

#[test]
fn ret_mismatch() {
    let other_f = {
        let locals = [<u64>::get_ptype()];
        let b0 = block!(
            assign(local(0), const_int::<u64>(0)),
            return_()
        );

        function(Ret::Yes, 0, &locals, &[b0])
    };

    let locals = [<u8>::get_ptype()];

    let b0 = block!(
        storage_live(0),
        call(1, &[], Some(local(0)), Some(1))
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f, other_f]);
    dump_program(p);
    assert_stop(p);
}