use crate::*;

/// The program written out:
/// fn main() {
///     let dp = allocate(sizeof *const ());
///     *dp = dp;
///     let id = spawn(second, *dp);
///     join(id);
/// }
/// 
/// fn second(dp) {
///     *dp = null;
/// }
/// 
/// This program should obviously not have a data race, but since
/// we do a trace based search it has one in the base. Atomicity::Init does not prevent this.
/// Extending Atomicity::Init to prevent this is ugly.
#[test]
fn weird_datapointer() {
    let pp_ptype = <*const *const ()>::get_ptype(); // Pointer pointer place type.
    let locals = [pp_ptype, <u32>::get_ptype()];

    let size = const_int::<usize>(<*const ()>::get_size().bytes());
    let align = const_int::<usize>(<*const ()>::get_align().bytes());

    let b0 = block!(
        storage_live(0),
        allocate(size, align, local(0), 1)
    );
    let b1 = block!(
        storage_live(1),
        assign(deref(load(local(0)), pp_ptype), load(local(0))),
        spawn(fn_ptr(1), load(deref(load(local(0)), pp_ptype)), Some(local(1)), 2)
    );
    let b2 = block!(
        join(load(local(1)), 3)
    );
    let b3 = block!( exit() );
    let main = function(Ret::No, 0, &locals, &[b0,b1,b2,b3]);

    let locals = [pp_ptype];
    let b0 = block!(
        assign(deref(load(local(0)), pp_ptype), null()),
        return_(),
    );
    let second = function(Ret::No, 1, &locals, &[b0]);

    let prog = program(&[main, second]);

    assert_stop(prog);
}
