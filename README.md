# Experiments

## Vanilla

Here a slow queryable (takes 1 second to respond) is connected to 80 peers that query it 5 times;
once every 20 seconds.

### Instructions

1. Create a directory called `vanilla`
2. Build the binaries using `cargo build --release`
3. Lunch the slow queryable using `./target/release/vanilla_slow_queryable vanilla.config.json5`
4. Open a python3 REPl and run `import vanilla; procs = vanilla.spawn_procs(80)`
5. Observe peer execution using `tail -f vanilla/<number>.stdout`
6. Observe slow queryable execution in (3)

### Results

The slow queryable gradually has its channel filled up, until it can no longer reply to any queries.
But the Zenoh runtime does not enter a deadlock state; the queryable continues to progress until the
channel size goes back to 0.

### Variations

#### Vanilla w/ `taskset -c 0`

Here we lunch the slow queryable with `taskset -c 0 ./target/release/vanilla_slow_queryable
vanilla.config.json5`. The slow queryable never sends out any reply.
The RX threads seem to be forever blocked on `flume::Sender::send`.

#### Vanilla w/ `taskset -c 0-1`

Here we lunch the slow queryable with `taskset -c 0-1 ./target/release/vanilla_slow_queryable
vanilla.config.json5`. The results are seemingly exactly the same as without `taskset -c 0-1`.

#### Vanilla w/ 10 queryables

Here we lunch the slow queryable with `taskset -c 0-1 ./target/release/vanilla_many_slow_queryables
vanilla.config.json5`. No queryable seems to reply to any peer. However, the backtraces don't show
signs of a deadlock; the only blocking operations seem to be the infamous `flume::Sender::send` on the RX threads.

<details>
<summary>Result of `thread apply bt all` in GDB</summary>

```text
Thread 13 (Thread 0x7f8ea614f640 (LWP 2989760) "net-2"):
# 0  0x00007f8ea7ca8e2e in epoll_wait (epfd=9, events=0x560169ffdc80, maxevents=1024, timeout=-1) at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
# 1  0x0000560169a2ec2e in mio::sys::unix::selector::epoll::Selector::select ()
# 2  0x0000560169a227e0 in _ZN5tokio7runtime2io6driver6Driver4turn17he5e3479e24668c79E.llvm.1089576002870961611 ()
# 3  0x0000560169a0b2f3 in _ZN5tokio7runtime4time6Driver13park_internal17hecd938c669d24b17E.llvm.17237780523312982448 ()
# 4  0x0000560169a21e3b in tokio::runtime::scheduler::multi_thread::park::Parker::park ()
# 5  0x0000560169a0a257 in tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout ()
# 6  0x0000560169a09314 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x000056016945ff51 in tokio::runtime::task::core::Core<T,S>::poll ()
# 10 0x0000560169461e7d in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 11 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 12 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 13 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 14 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 15 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 17 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 18 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 12 (Thread 0x7f8ea655a640 (LWP 2988420) "rx-1"):
# 0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
# 1  0x0000560169a46944 in std::sys::unix::futex::futex_wait () at library/std/src/sys/unix/futex.rs:62
# 2  std::sys_common::thread_parking::futex::Parker::park () at library/std/src/sys_common/thread_parking/futex.rs:52
# 3  std::thread::park () at library/std/src/thread/mod.rs:1066
# 4  0x00005601690e3465 in _ZN128_$LT$$LP$flume..Sender$LT$T$GT$$C$flume..Receiver$LT$T$GT$$RP$$u20$as$u20$zenoh..handlers..IntoCallbackReceiverPair$LT$T$GT$$GT$21into_cb_receiver_pair28_$u7b$$u7b$closure$u7d$$u7d$17he5d5af46758de164E.llvm.7076424268960255647 ()
# 5  0x000056016917ae0c in zenoh::session::Session::handle_query ()
# 6  0x000056016917e306 in <zenoh::session::Session as zenoh::net::primitives::Primitives>::send_request ()
# 7  0x00005601690ffa9f in <zenoh::session::Session as zenoh::net::primitives::EPrimitives>::send_request ()
# 8  0x000056016918d59f in zenoh::net::routing::dispatcher::queries::route_query ()
# 9  0x0000560169123341 in <zenoh::net::routing::dispatcher::face::Face as zenoh::net::primitives::Primitives>::send_request ()
# 10 0x00005601691fe19a in <zenoh::net::primitives::demux::DeMux as zenoh_transport::TransportPeerEventHandler>::handle_message ()
# 11 0x00005601692124fe in <zenoh::net::runtime::RuntimeSession as zenoh_transport::TransportPeerEventHandler>::handle_message ()
# 12 0x000056016938bf2c in zenoh_transport::unicast::universal::rx::<impl zenoh_transport::unicast::universal::transport::TransportUnicastUniversal>::read_messages ()
# 13 0x00005601693dd361 in _ZN103_$LT$tokio_util..task..task_tracker..TrackedFuture$LT$F$GT$$u20$as$u20$core..future..future..Future$GT$4poll17hffd4b8dbc78ae40fE.llvm.9775743301247060175 ()
# 14 0x00005601693e9ed2 in tokio::runtime::task::core::Core<T,S>::poll ()
# 15 0x000056016940506f in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 16 0x0000560169a09e1b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 17 0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 18 0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 19 0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 20 0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 21 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 22 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 23 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 24 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 25 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 26 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 27 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 28 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 29 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 30 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 11 (Thread 0x7f8ea675b640 (LWP 2988419) "rx-0"):
# 0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
# 1  0x0000560169a46944 in std::sys::unix::futex::futex_wait () at library/std/src/sys/unix/futex.rs:62
# 2  std::sys_common::thread_parking::futex::Parker::park () at library/std/src/sys_common/thread_parking/futex.rs:52
# 3  std::thread::park () at library/std/src/thread/mod.rs:1066
# 4  0x00005601690e3465 in _ZN128_$LT$$LP$flume..Sender$LT$T$GT$$C$flume..Receiver$LT$T$GT$$RP$$u20$as$u20$zenoh..handlers..IntoCallbackReceiverPair$LT$T$GT$$GT$21into_cb_receiver_pair28_$u7b$$u7b$closure$u7d$$u7d$17he5d5af46758de164E.llvm.7076424268960255647 ()
# 5  0x000056016917ae0c in zenoh::session::Session::handle_query ()
# 6  0x000056016917e306 in <zenoh::session::Session as zenoh::net::primitives::Primitives>::send_request ()
# 7  0x00005601690ffa9f in <zenoh::session::Session as zenoh::net::primitives::EPrimitives>::send_request ()
# 8  0x000056016918d59f in zenoh::net::routing::dispatcher::queries::route_query ()
# 9  0x0000560169123341 in <zenoh::net::routing::dispatcher::face::Face as zenoh::net::primitives::Primitives>::send_request ()
# 10 0x00005601691fe19a in <zenoh::net::primitives::demux::DeMux as zenoh_transport::TransportPeerEventHandler>::handle_message ()
# 11 0x00005601692124fe in <zenoh::net::runtime::RuntimeSession as zenoh_transport::TransportPeerEventHandler>::handle_message ()
# 12 0x000056016938bf2c in zenoh_transport::unicast::universal::rx::<impl zenoh_transport::unicast::universal::transport::TransportUnicastUniversal>::read_messages ()
# 13 0x00005601693dd361 in _ZN103_$LT$tokio_util..task..task_tracker..TrackedFuture$LT$F$GT$$u20$as$u20$core..future..future..Future$GT$4poll17hffd4b8dbc78ae40fE.llvm.9775743301247060175 ()
# 14 0x00005601693e9ed2 in tokio::runtime::task::core::Core<T,S>::poll ()
# 15 0x000056016940506f in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 16 0x0000560169a09e1b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 17 0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 18 0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 19 0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 20 0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 21 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 22 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 23 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 24 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 25 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 26 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 27 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 28 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 29 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 30 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 10 (Thread 0x7f8ea695c640 (LWP 2988418) "tx-0"):
# 0  0x00007f8ea7ca8e2e in epoll_wait (epfd=18, events=0x7f8e7400a400, maxevents=1024, timeout=20) at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
# 1  0x0000560169a2ec2e in mio::sys::unix::selector::epoll::Selector::select ()
# 2  0x0000560169a227e0 in _ZN5tokio7runtime2io6driver6Driver4turn17he5e3479e24668c79E.llvm.1089576002870961611 ()
# 3  0x0000560169a0b2f3 in_ZN5tokio7runtime4time6Driver13park_internal17hecd938c669d24b17E.llvm.17237780523312982448 ()
# 4  0x0000560169a21e3b in tokio::runtime::scheduler::multi_thread::park::Parker::park ()
# 5  0x0000560169a0a257 in tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout ()
# 6  0x0000560169a09314 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 9 (Thread 0x7f8ea6b60640 (LWP 2988348) "acc-0"):
# 0  0x00007f8ea7ca8e2e in epoll_wait (epfd=13, events=0x56016a004b70, maxevents=1024, timeout=-1) at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
# 1  0x0000560169a2ec2e in mio::sys::unix::selector::epoll::Selector::select ()
# 2  0x0000560169a227e0 in _ZN5tokio7runtime2io6driver6Driver4turn17he5e3479e24668c79E.llvm.1089576002870961611 ()
# 3  0x0000560169a0b2f3 in_ZN5tokio7runtime4time6Driver13park_internal17hecd938c669d24b17E.llvm.17237780523312982448 ()
# 4  0x0000560169a21e3b in tokio::runtime::scheduler::multi_thread::park::Parker::park ()
# 5  0x0000560169a0a257 in tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout ()
# 6  0x0000560169a09314 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 8 (Thread 0x7f8ea716c640 (LWP 2988345) "app-0"):
# 0  0x00007f8ea7ca8e2e in epoll_wait (epfd=6, events=0x560169ff8720, maxevents=1024, timeout=-1) at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
# 1  0x0000560169a2ec2e in mio::sys::unix::selector::epoll::Selector::select ()
# 2  0x0000560169a227e0 in _ZN5tokio7runtime2io6driver6Driver4turn17he5e3479e24668c79E.llvm.1089576002870961611 ()
# 3  0x0000560169a0b2f3 in_ZN5tokio7runtime4time6Driver13park_internal17hecd938c669d24b17E.llvm.17237780523312982448 ()
# 4  0x0000560169a21e3b in tokio::runtime::scheduler::multi_thread::park::Parker::park ()
# 5  0x0000560169a0a257 in tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout ()
# 6  0x0000560169a09314 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 7 (Thread 0x7f8ea7376640 (LWP 2988344) "tokio-runtime-w"):
# 0  0x00005601690e9702 in flume::Receiver<T>::len ()
# 1  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 2  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 3  0x00005601690bd38a in std::panicking::try ()
# 4  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 5  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 6  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 6 (Thread 0x7f8ea757a640 (LWP 2988343) "tokio-runtime-w"):
# 0  0x00005601690e9760 in flume::Receiver<T>::len ()
# 1  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 2  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 3  0x00005601690bd38a in std::panicking::try ()
# 4  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 5  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 6  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 5 (Thread 0x7f8e9f57a640 (LWP 2988342) "tokio-runtime-w"):
# 0  0x00005601690e7c77 in _ZN5flume13Chan$LT$T$GT$12pull_pending17h56f46eec12f42a58E.llvm.7076424268960255647 ()
# 1  0x00005601690e9737 in flume::Receiver<T>::len ()
# 2  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 3  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 4  0x00005601690bd38a in std::panicking::try ()
# 5  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 6  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 7  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 8  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 9  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 10 0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 11 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 12 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 13 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 14 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 15 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 16 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 18 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 19 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 20 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 4 (Thread 0x7f8ea777e640 (LWP 2988341) "tokio-runtime-w"):
# 0  0x00005601690e96e7 in flume::Receiver<T>::len ()
# 1  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 2  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 3  0x00005601690bd38a in std::panicking::try ()
# 4  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 5  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 6  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 3 (Thread 0x7f8ea797f640 (LWP 2988340) "tokio-runtime-w"):
# 0  0x00005601690e9737 in flume::Receiver<T>::len ()
# 1  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 2  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 3  0x00005601690bd38a in std::panicking::try ()
# 4  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 5  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 6  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 7  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 8  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 9  0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 10 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 11 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 12 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 13 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 14 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 15 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 16 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 18 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 19 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 2 (Thread 0x7f8ea7b80640 (LWP 2988339) "tokio-runtime-w"):
# 0  0x00005601690e7c41 in _ZN5flume13Chan$LT$T$GT$12pull_pending17h56f46eec12f42a58E.llvm.7076424268960255647 ()
# 1  0x00005601690e9737 in flume::Receiver<T>::len ()
# 2  0x00005601690f7558 in _ZN28vanilla_many_slow_queryables17declare_queryable28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfcfe8a70b347a105E.llvm.677274606435115739 ()
# 3  0x00005601690f5cfe in tokio::runtime::task::core::Core<T,S>::poll ()
# 4  0x00005601690bd38a in std::panicking::try ()
# 5  0x000056016909a70a in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 6  0x0000560169a09f4b in tokio::runtime::scheduler::multi_thread::worker::Context::run_task ()
# 7  0x0000560169a08c94 in tokio::runtime::scheduler::multi_thread::worker::Context::run ()
# 8  0x0000560169a26ba8 in tokio::runtime::context::runtime::enter_runtime ()
# 9  0x0000560169a08638 in tokio::runtime::scheduler::multi_thread::worker::run ()
# 10 0x0000560169a0edd3 in <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll ()
# 11 0x0000560169a142f7 in tokio::runtime::task::core::Core<T,S>::poll ()
# 12 0x0000560169a04eaa in tokio::runtime::task::harness::Harness<T,S>::poll ()
# 13 0x0000560169a0e44c in tokio::runtime::blocking::pool::Inner::run ()
# 14 0x0000560169a24bf7 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 15 0x0000560169a25549 in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
# 16 0x0000560169a52eb5 in alloc::boxed::{impl#47}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 17 alloc::boxed::{impl#47}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:2015
# 18 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
# 19 0x00007f8ea7c17ac3 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
# 20 0x00007f8ea7ca9850 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81

Thread 1 (Thread 0x7f8ea7b81d80 (LWP 2988338) "vanilla_many_sl"):
# 0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
# 1  0x0000560169a53951 in std::sys::unix::futex::futex_wait () at library/std/src/sys/unix/futex.rs:62
# 2  std::sys::unix::locks::futex_condvar::Condvar::wait_optional_timeout () at library/std/src/sys/unix/locks/futex_condvar.rs:49
# 3  std::sys::unix::locks::futex_condvar::Condvar::wait () at library/std/src/sys/unix/locks/futex_condvar.rs:33
# 4  0x0000560169a1d8b0 in _ZN5tokio7runtime4park5Inner4park17hfd763b1a8184afcdE.llvm.7103791555393378285 ()
# 5  0x00005601690640ff in tokio::runtime::park::CachedParkThread::block_on ()
# 6  0x00005601690de086 in tokio::runtime::runtime::Runtime::block_on ()
# 7  0x00005601690c4692 in vanilla_many_slow_queryables::main ()
# 8  0x0000560169101ef3 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
# 9  0x0000560169101f09 in _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he2467fccdbbe82c1E.llvm.3019940485211690251 ()
# 10 0x0000560169a462e1 in core::ops::function::impls::{impl#2}::call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> () at library/core/src/ops/function.rs:284
# 11 std::panicking::try::do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> () at library/std/src/panicking.rs:552
# 12 std::panicking::try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> () at library/std/src/panicking.rs:516
# 13 std::panic::catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> () at library/std/src/panic.rs:142
# 14 std::rt::lang_start_internal::{closure#2} () at library/std/src/rt.rs:148
# 15 std::panicking::try::do_call<std::rt::lang_start_internal::{closure_env#2}, isize> () at library/std/src/panicking.rs:552
# 16 std::panicking::try<isize, std::rt::lang_start_internal::{closure_env#2}> () at library/std/src/panicking.rs:516
# 17 std::panic::catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> () at library/std/src/panic.rs:142
# 18 std::rt::lang_start_internal () at library/std/src/rt.rs:148
# 19 0x00005601690c4775 in main ()
```

</details>

### System information

Ubuntu 22.04.4 LTS (Linux 5.15.0-25-generic), Intel(R) Xeon(R) CPU E5-2630 v4 @ 2.20GHz, 16G RAM
