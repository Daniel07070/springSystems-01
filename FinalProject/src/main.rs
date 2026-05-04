use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// -------------------- TASK --------------------

#[derive(Debug, Clone)]
enum TaskType {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    arrival_time: u64,
    duration: u64,
    kind: TaskType,
    cpu_cost: u64,
}

// -------------------- TASK GENERATOR --------------------

fn generate_tasks(n: usize) -> Vec<Task> {
    let mut tasks = Vec::new();

    for i in 0..n {
        let (kind, cpu_cost) = if i % 10 < 7 {
            (TaskType::IO, 10)
        } else {
            (TaskType::CPU, 35)
        };

        tasks.push(Task {
            id: i,
            arrival_time: i as u64 * 20,
            duration: 200,
            kind,
            cpu_cost,
        });
    }

    tasks
}

// -------------------- STATS --------------------

#[derive(Debug, Default)]
struct Stats {
    total_time: u64,
    busy_time: u64,
    completed_tasks: u64,
}

// -------------------- MAIN --------------------

fn main() {
    let tasks = generate_tasks(1000);

    // worker -> manager (ONLY completion reports)
    let (tx_done, rx_done) = mpsc::channel::<Task>();

    // manager -> workers (dispatch channel)
    let (tx_work, rx_work) = mpsc::channel::<Task>();

    let rx_work = Arc::new(Mutex::new(rx_work));
    let stats = Arc::new(Mutex::new(Stats::default()));

    // =====================================================
    // MONITOR THREAD
    // =====================================================
    {
        let stats = Arc::clone(&stats);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));

                let s = stats.lock().unwrap();

                let utilization = if s.total_time > 0 {
                    (s.busy_time as f64 / s.total_time as f64) * 100.0
                } else {
                    0.0
                };

                println!("\n===== MONITOR =====");
                println!("Completed tasks: {}", s.completed_tasks);
                println!("Busy time: {}", s.busy_time);
                println!("Total time: {}", s.total_time);
                println!("CPU usage: {:.2}%", utilization);
                println!("===================\n");
            }
        });
    }

    // =====================================================
    // WORKER POOL (8 WORKERS)
    // =====================================================
    for worker_id in 0..8 {
        let rx_work = Arc::clone(&rx_work);
        let tx_done = tx_done.clone();
        let stats = Arc::clone(&stats);

        thread::spawn(move || {
            loop {
                let task = {
                    let lock = rx_work.lock().unwrap();
                    lock.recv()
                };

                match task {
                    Ok(mut task) => {
                        println!(
                            "Worker {} running task {} Type: {:?}",
                            worker_id, task.id, task.kind
                        );

                        let run_time = task.cpu_cost;
                        task.cpu_cost = 0;

                        thread::sleep(Duration::from_millis(1));

                        {
                            let mut s = stats.lock().unwrap();
                            s.busy_time += run_time;
                        }

                        {
                            let mut s = stats.lock().unwrap();
                            s.completed_tasks += 1;
                        }

                        println!(
                            "Worker {} finished task {} Type {:?}",
                            worker_id, task.id, task.kind
                        );

                        // ONLY report completion (no requeue)
                        tx_done.send(task).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
    }

    // =====================================================
    // MANAGER (SOLE SCHEDULER)
    // =====================================================
    let tx_work = tx_work.clone();
    let stats_manager = Arc::clone(&stats);

    thread::spawn(move || {
        let mut ready_queue: VecDeque<Task> = VecDeque::new();
        let mut time: u64 = 0;
        let mut i = 0;

        let mut cpu_used = 0.0;

        loop {
            time += 1;

            // reset CPU window every 100 ticks
            if time % 100 == 0 {
                cpu_used = 0.0;
            }

            // admit tasks
            while i < tasks.len() && tasks[i].arrival_time <= time {
                ready_queue.push_back(tasks[i].clone());
                i += 1;
            }

            {
                let mut s = stats_manager.lock().unwrap();
                s.total_time = time;
            }

            // receive completed tasks (no requeue anymore, just accounting)
            while let Ok(_task) = rx_done.try_recv() {
                // no requeue needed in strict model
            }

            // schedule next task
            if let Some(task) = ready_queue.pop_front() {
                let cost = match task.kind {
                    TaskType::CPU => 35.0,
                    TaskType::IO => 10.0,
                };

                if cpu_used + cost <= 100.0 {
                    cpu_used += cost;
                    tx_work.send(task).unwrap();
                } else {
                    ready_queue.push_back(task);
                }
            }

            if i >= tasks.len() && ready_queue.is_empty() {
                break;
            }

            thread::sleep(Duration::from_millis(1));
        }
    });

    // keep program alive
    loop {
        thread::park();
    }
}