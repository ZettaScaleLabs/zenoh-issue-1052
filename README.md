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

## Replication

Here a router with a filesystem storage containing 6K files is connected to 50 peers with filesystem
storages subscribing to the same key expression. All nodes use Zenoh 0.11.0.

### Instructions

1. Build the binary using `cargo build --release --bin replication_put`
2. Lunch the main router using `zenohd -c replication.config.json5`
3. Open a python3 REPL and run `import replication; e = replication.Experiment(50)`
4. Observe peer execution using `tail -f replication/<number>.peer.replication.stdout`

### Results

No storages make progress on replication. Logs show that many sent/received query replies don't have an associated query (the error messages are of the form `Received ReplyData for unkown Query: 20914` and `Route final reply Face{3, cc6d728d780b29ddad1a4b6e9dd4a191}:9 from Face{3, cc6d728d780b29ddad1a4b6e9dd4a191}: Query nof found!`).

<details>
<summary>Result of `thread apply bt all` in GDB</summary>

```text
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = signal SIGSTOP
  * frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x000000010514ce7c zenohd`tokio::runtime::park::Inner::park::h4ecaab8bc3bc595b + 280
    frame #3: 0x0000000104e929a4 zenohd`zenohd::main::h38e2e8e7ae24c0c6 + 10160
    frame #4: 0x0000000104dee798 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::he66a75b06fe9e38b + 12
    frame #5: 0x0000000104ea08d0 zenohd`main + 580
    frame #6: 0x000000018d8b50e0 dyld`start + 2360
  thread #2, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #3, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010514df8c zenohd`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 544
    frame #2: 0x000000010514dc4c zenohd`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 744
    frame #3: 0x0000000105152460 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 828
    frame #4: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #5: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #6: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #7: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #8: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #4, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #5, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #6, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #7, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #8, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #9, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000105152388 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 612
    frame #3: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #4: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #5: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #6: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #7: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #10, name = 'app-0'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010514df8c zenohd`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 544
    frame #2: 0x000000010514dc4c zenohd`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 744
    frame #3: 0x0000000105152460 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 828
    frame #4: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #5: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #6: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #7: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #8: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #11, name = 'net-0'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010514df8c zenohd`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 544
    frame #2: 0x000000010514dc4c zenohd`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 744
    frame #3: 0x0000000105152460 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 828
    frame #4: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #5: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #6: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #7: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #8: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #12, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010645cfe4 libzenoh_plugin_storage_manager.dylib`mio::poll::Poll::poll::h54374a4211b12479 + 116
    frame #2: 0x0000000106453dc0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 432
    frame #3: 0x0000000106454fa0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 680
    frame #4: 0x0000000106452d74 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 796
    frame #5: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #6: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #7: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #8: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #9: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #10: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #11: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #12: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #13, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #14, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #15, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #16, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #17, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #18, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #19, name = 'tokio-runtime-worker'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106452cb0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 600
    frame #3: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #4: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #5: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #6: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #7: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #9: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #10: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #20, name = 'async-global-executor/tokio'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x000000010644df68 libzenoh_plugin_storage_manager.dylib`tokio::runtime::park::Inner::park::h4ecaab8bc3bc595b + 304
    frame #3: 0x000000010644099c libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h9f8278e1e2b08271 + 440
    frame #4: 0x0000000106441f20 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hefcf8905ffa99d61 + 136
    frame #5: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #6: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #21, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001064692a8 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 332
    frame #3: 0x0000000106447d10 libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 3264
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #22, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001064692a8 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 332
    frame #3: 0x00000001064474fc libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 1196
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #23, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001064692a8 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 332
    frame #3: 0x00000001064474fc libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 1196
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #24, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001064692a8 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 332
    frame #3: 0x00000001064474fc libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 1196
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #25, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001064692a8 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 332
    frame #3: 0x00000001064474fc libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 1196
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #26, name = 'async-io'
    frame #0: 0x000000018dbf8524 libsystem_kernel.dylib`__psynch_mutexwait + 8
    frame #1: 0x000000018dc33168 libsystem_pthread.dylib`_pthread_mutex_firstfit_lock_wait + 84
    frame #2: 0x000000018dc30af8 libsystem_pthread.dylib`_pthread_mutex_firstfit_lock_slow + 248
    frame #3: 0x000000010646052c libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::hcbb08ab697547c75 + 180
    frame #4: 0x0000000106460918 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h86322e91f81a3c97 + 100
    frame #5: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #6: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #8: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #27, name = 'async-std/runtime'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x0000000106466318 libzenoh_plugin_storage_manager.dylib`polling::Poller::wait::h1a85dcfa9debac68 + 612
    frame #2: 0x0000000106465190 libzenoh_plugin_storage_manager.dylib`async_io::reactor::ReactorLock::react::hd554c8b00122a692 + 204
    frame #3: 0x0000000106447a64 libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 2580
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #28, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106469278 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 284
    frame #3: 0x0000000106447d10 libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 3264
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #29, name = 'async-std/runtime'
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x0000000106469278 libzenoh_plugin_storage_manager.dylib`parking::Inner::park::h079c767b3e1422bc + 284
    frame #3: 0x0000000106447d10 libzenoh_plugin_storage_manager.dylib`async_global_executor::threading::thread_main_loop::h98d37384217a9e08 + 3264
    frame #4: 0x00000001064407dc libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h8d9400638e2459c8 + 12
    frame #5: 0x0000000106441e18 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd02a9e4a0641c4c7 + 96
    frame #6: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #7: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #8: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #30
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x000000018db5d4dc libc++.1.dylib`std::__1::condition_variable::wait(std::__1::unique_lock<std::__1::mutex>&) + 28
    frame #3: 0x000000010758cb10 libzenoh_backend_fs.dylib`rocksdb::ThreadPoolImpl::Impl::BGThread(unsigned long) + 304
    frame #4: 0x000000010758ce78 libzenoh_backend_fs.dylib`rocksdb::ThreadPoolImpl::Impl::BGThreadWrapper(void*) + 124
    frame #5: 0x000000010758e74c libzenoh_backend_fs.dylib`void* std::__1::__thread_proxy[abi:v160006]<std::__1::tuple<std::__1::unique_ptr<std::__1::__thread_struct, std::__1::default_delete<std::__1::__thread_struct>>, void (*)(void*), rocksdb::BGThreadMetadata*>>(void*) + 52
    frame #6: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #31
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x000000018db5d4dc libc++.1.dylib`std::__1::condition_variable::wait(std::__1::unique_lock<std::__1::mutex>&) + 28
    frame #3: 0x000000010758cb10 libzenoh_backend_fs.dylib`rocksdb::ThreadPoolImpl::Impl::BGThread(unsigned long) + 304
    frame #4: 0x000000010758ce78 libzenoh_backend_fs.dylib`rocksdb::ThreadPoolImpl::Impl::BGThreadWrapper(void*) + 124
    frame #5: 0x000000010758e74c libzenoh_backend_fs.dylib`void* std::__1::__thread_proxy[abi:v160006]<std::__1::tuple<std::__1::unique_ptr<std::__1::__thread_struct, std::__1::default_delete<std::__1::__thread_struct>>, void (*)(void*), rocksdb::BGThreadMetadata*>>(void*) + 52
    frame #6: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #32
    frame #0: 0x000000018dbf906c libsystem_kernel.dylib`__psynch_cvwait + 8
    frame #1: 0x000000018dc365fc libsystem_pthread.dylib`_pthread_cond_wait + 1228
    frame #2: 0x00000001074ee530 libzenoh_backend_fs.dylib`rocksdb::port::CondVar::TimedWait(unsigned long long) + 72
    frame #3: 0x000000010748c0fc libzenoh_backend_fs.dylib`rocksdb::InstrumentedCondVar::TimedWait(unsigned long long) + 240
    frame #4: 0x00000001073b798c libzenoh_backend_fs.dylib`rocksdb::Timer::Run() + 180
    frame #5: 0x00000001073b7ebc libzenoh_backend_fs.dylib`void* std::__1::__thread_proxy[abi:v160006]<std::__1::tuple<std::__1::unique_ptr<std::__1::__thread_struct, std::__1::default_delete<std::__1::__thread_struct>>, void (rocksdb::Timer::*)(), rocksdb::Timer*>>(void*) + 72
    frame #6: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #33, name = 'net-0'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010645cfe4 libzenoh_plugin_storage_manager.dylib`mio::poll::Poll::poll::h54374a4211b12479 + 116
    frame #2: 0x0000000106453dc0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 432
    frame #3: 0x0000000106454fa0 libzenoh_plugin_storage_manager.dylib`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 680
    frame #4: 0x0000000106452d74 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 796
    frame #5: 0x0000000106452124 libzenoh_plugin_storage_manager.dylib`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 4728
    frame #6: 0x00000001064563dc libzenoh_plugin_storage_manager.dylib`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 440
    frame #7: 0x000000010644a8ec libzenoh_plugin_storage_manager.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 492
    frame #8: 0x000000010644b5a8 libzenoh_plugin_storage_manager.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 112
    frame #9: 0x000000010648a12c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h6afbab2daea47eae at boxed.rs:1993:9 [opt]
    frame #10: 0x000000010648a120 libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h04503fef482f2258 at boxed.rs:1993:9 [opt]
    frame #11: 0x000000010648a11c libzenoh_plugin_storage_manager.dylib`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c at thread.rs:108:17 [opt]
    frame #12: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #34, name = 'acc-0'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010514df8c zenohd`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 544
    frame #2: 0x000000010514dc4c zenohd`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 744
    frame #3: 0x0000000105152460 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 828
    frame #4: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #5: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #6: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #7: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #8: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #35, name = 'tx-0'
    frame #0: 0x000000018dbfb9c0 libsystem_kernel.dylib`kevent + 8
    frame #1: 0x000000010514df8c zenohd`tokio::runtime::io::driver::Driver::turn::ha16b0a73be994533 + 544
    frame #2: 0x000000010514dc4c zenohd`tokio::runtime::time::Driver::park_internal::hfa1f277a9394f133 + 744
    frame #3: 0x0000000105152460 zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::park_timeout::h801c44da3feda799 + 828
    frame #4: 0x0000000105151544 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 6080
    frame #5: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #6: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #7: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #8: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #9: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #36, name = 'rx-0'
    frame #0: 0x000000018dbf57f0 libsystem_kernel.dylib`semaphore_wait_trap + 8
    frame #1: 0x000000018da84eac libdispatch.dylib`_dispatch_sema4_wait + 28
    frame #2: 0x000000018da8555c libdispatch.dylib`_dispatch_semaphore_wait_slow + 132
    frame #3: 0x0000000106481274 libzenoh_plugin_storage_manager.dylib`std::thread::park::heb7297ced8569b12 [inlined] std::sys::unix::thread_parking::darwin::Parker::park::heb53f2352b15461b at darwin.rs:75:15 [opt]
    frame #4: 0x0000000106481254 libzenoh_plugin_storage_manager.dylib`std::thread::park::heb7297ced8569b12 at mod.rs:988:9 [opt]
    frame #5: 0x000000010625de70 libzenoh_plugin_storage_manager.dylib`_$LT$$LP$flume..Sender$LT$T$GT$$C$flume..Receiver$LT$T$GT$$RP$$u20$as$u20$zenoh..handlers..IntoCallbackReceiverPair$LT$T$GT$$GT$::into_cb_receiver_pair::_$u7b$$u7b$closure$u7d$$u7d$::hb9c939c83e495649 + 2068
    frame #6: 0x000000010630ce80 libzenoh_plugin_storage_manager.dylib`zenoh::session::Session::handle_query::h10b0aa1da465ac19 + 1824
    frame #7: 0x000000010630f4a4 libzenoh_plugin_storage_manager.dylib`_$LT$zenoh..session..Session$u20$as$u20$zenoh..net..primitives..Primitives$GT$::send_request::h4fb72a0a9d0f6c48 + 800
    frame #8: 0x0000000106313090 libzenoh_plugin_storage_manager.dylib`_$LT$zenoh..session..Session$u20$as$u20$zenoh..net..primitives..EPrimitives$GT$::send_request::hee14d8af4e37f50a + 52
    frame #9: 0x00000001051e09c0 zenohd`zenoh::net::routing::dispatcher::queries::route_query::hfe67b623f89a2436 + 7284
    frame #10: 0x00000001051c8510 zenohd`_$LT$zenoh..net..routing..dispatcher..face..Face$u20$as$u20$zenoh..net..primitives..Primitives$GT$::send_request::ha443a5b943bde6c2 + 172
    frame #11: 0x000000010532c8bc zenohd`_$LT$zenoh..net..primitives..demux..DeMux$u20$as$u20$zenoh_transport..TransportPeerEventHandler$GT$::handle_message::h05c55ffa741c657d + 1532
    frame #12: 0x00000001054dffb4 zenohd`_$LT$tokio_util..task..task_tracker..TrackedFuture$LT$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hbd274cd68de3412d + 7468
    frame #13: 0x00000001054dc3c4 zenohd`tokio::runtime::task::raw::poll::he927c67a6c0dd820 + 468
    frame #14: 0x0000000105152afc zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h71fbede03b2fd8aa + 500
    frame #15: 0x0000000105151210 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 5260
    frame #16: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #17: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #18: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #19: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #20: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
  thread #37, name = 'rx-1'
    frame #0: 0x000000018dbf57f0 libsystem_kernel.dylib`semaphore_wait_trap + 8
    frame #1: 0x000000018da84eac libdispatch.dylib`_dispatch_sema4_wait + 28
    frame #2: 0x000000018da8555c libdispatch.dylib`_dispatch_semaphore_wait_slow + 132
    frame #3: 0x0000000106481274 libzenoh_plugin_storage_manager.dylib`std::thread::park::heb7297ced8569b12 [inlined] std::sys::unix::thread_parking::darwin::Parker::park::heb53f2352b15461b at darwin.rs:75:15 [opt]
    frame #4: 0x0000000106481254 libzenoh_plugin_storage_manager.dylib`std::thread::park::heb7297ced8569b12 at mod.rs:988:9 [opt]
    frame #5: 0x000000010625de70 libzenoh_plugin_storage_manager.dylib`_$LT$$LP$flume..Sender$LT$T$GT$$C$flume..Receiver$LT$T$GT$$RP$$u20$as$u20$zenoh..handlers..IntoCallbackReceiverPair$LT$T$GT$$GT$::into_cb_receiver_pair::_$u7b$$u7b$closure$u7d$$u7d$::hb9c939c83e495649 + 2068
    frame #6: 0x000000010630ce80 libzenoh_plugin_storage_manager.dylib`zenoh::session::Session::handle_query::h10b0aa1da465ac19 + 1824
    frame #7: 0x000000010630f4a4 libzenoh_plugin_storage_manager.dylib`_$LT$zenoh..session..Session$u20$as$u20$zenoh..net..primitives..Primitives$GT$::send_request::h4fb72a0a9d0f6c48 + 800
    frame #8: 0x0000000106313090 libzenoh_plugin_storage_manager.dylib`_$LT$zenoh..session..Session$u20$as$u20$zenoh..net..primitives..EPrimitives$GT$::send_request::hee14d8af4e37f50a + 52
    frame #9: 0x00000001051e09c0 zenohd`zenoh::net::routing::dispatcher::queries::route_query::hfe67b623f89a2436 + 7284
    frame #10: 0x00000001051c8510 zenohd`_$LT$zenoh..net..routing..dispatcher..face..Face$u20$as$u20$zenoh..net..primitives..Primitives$GT$::send_request::ha443a5b943bde6c2 + 172
    frame #11: 0x000000010532c8bc zenohd`_$LT$zenoh..net..primitives..demux..DeMux$u20$as$u20$zenoh_transport..TransportPeerEventHandler$GT$::handle_message::h05c55ffa741c657d + 1532
    frame #12: 0x00000001054dffb4 zenohd`_$LT$tokio_util..task..task_tracker..TrackedFuture$LT$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hbd274cd68de3412d + 7468
    frame #13: 0x00000001054dc3c4 zenohd`tokio::runtime::task::raw::poll::he927c67a6c0dd820 + 468
    frame #14: 0x0000000105152afc zenohd`tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h71fbede03b2fd8aa + 500
    frame #15: 0x0000000105151210 zenohd`tokio::runtime::scheduler::multi_thread::worker::run::h5ffa83a70a59d44b + 5260
    frame #16: 0x00000001051585e8 zenohd`tokio::runtime::task::raw::poll::h1dca2030075632b5 + 724
    frame #17: 0x0000000105147230 zenohd`std::sys_common::backtrace::__rust_begin_short_backtrace::h7347c99c1faf3337 + 508
    frame #18: 0x0000000105146f60 zenohd`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2834d56fb80041a5 + 288
    frame #19: 0x0000000105143620 zenohd`std::sys::unix::thread::Thread::new::thread_start::h143a83d9ede86d5c + 48
    frame #20: 0x000000018dc36034 libsystem_pthread.dylib`_pthread_start + 136
```

</details>

### System information

Ubuntu 22.04.4 LTS (Linux 5.15.0-25-generic), Intel(R) Xeon(R) CPU E5-2630 v4 @ 2.20GHz, 16G RAM
