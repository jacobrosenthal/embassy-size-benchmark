#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

use panic_abort as _;

#[embassy::task]
async fn one() {
    loop {
        #[cfg(feature = "join")]
        {
            let join1 = async { loop {} };

            let join2 = async { loop {} };

            futures::join!(join1, join2);
        }

        #[cfg(not(feature = "join"))]
        {
            async { loop {} }.await;
        }
    }
}

#[cfg(feature = "spawn")]
#[embassy::task]
async fn two() {
    loop {}
}

#[cfg(feature = "executor")]
#[embassy::task]
async fn three() {
    loop {}
}

#[cfg(feature = "executor-interrupt")]
#[embassy::task]
async fn four() {
    loop {}
}

#[cfg(feature = "executor")]
static EXECUTOR_MED: embassy::util::Forever<
    embassy::executor::InterruptExecutor<embassy_nrf::interrupt::SWI0_EGU0>,
> = embassy::util::Forever::new();
static EXECUTOR_LOW: embassy::util::Forever<embassy::executor::Executor> =
    embassy::util::Forever::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let _p = embassy_nrf::init(Default::default());

    #[cfg(feature = "executor")]
    {
        use embassy::interrupt::InterruptExt;
        use embassy_nrf::interrupt;
        let irq = interrupt::take!(SWI0_EGU0);
        irq.set_priority(embassy_nrf::interrupt::Priority::P7);
        let executor = EXECUTOR_MED.put(embassy::executor::InterruptExecutor::new(irq));
        executor.start(|spawner| {
            let _ = spawner.spawn(three());
        });
    }

    let executor = EXECUTOR_LOW.put(embassy::executor::Executor::new());
    executor.run(|spawner| {
        let _ = spawner.spawn(one());
        #[cfg(feature = "spawn")]
        {
            let _ = spawner.spawn(two());
        }
    });
}
