#Task Dispatcher Final Project
#Steps to build and run:
1.) Create new codespace on local device
2.) Open new terminal and enter cd FinalProject/
3.) Once in the right directory, run the command cargo build
4.) Lastly, run cargo run and the program will finish by itself once all tasks are executed
(If you'd like to stop the program early hit ctrl+c)

#Commands
While there aren't any commands for users to run in their terminal, they can change the workload for IO and CPU
 for i in 0..n {
        let (kind, cpu_cost) = if i % 10 < 7 {
            (TaskType::IO, 10)
        } else {
            (TaskType::CPU, 35)
        };
-Go to the code segment above in the task generator section of the code(labeled)
Right now the distribution is 700 IO tasks and 300 CPU. All you have to do is change the number for if i % 10 < __
Whatever you enter is the percent distribution for IO tasks.

1. Design Summary (Architecture, Concurrency, and Scheduling)

The system is built around three main concurrent components: a manager (scheduler) thread, a worker pool, and a monitor thread.

The manager thread acts as the central scheduler. It maintains a FIFO ready queue that stores tasks as they arrive. It is responsible for admitting tasks into the system, deciding when they should be dispatched, and sending them to the worker pool through a dispatch channel. The manager does not reinsert completed tasks back into the queue, since each task is executed exactly once. It also tracks global simulation time and coordinates scheduling decisions based on CPU usage constraints.

The worker pool consists of eight concurrent worker threads. Each worker continuously receives tasks from the manager through a shared channel and executes them independently. Execution is simulated using sleep based on task duration. Once a task is completed, the worker sends a completion message back to the manager through a separate channel for accounting and metric updates.

The monitor thread is responsible for observing system state during execution. It periodically reads shared statistics and prints runtime information such as total completed tasks, CPU and IO task counts, CPU utilization, and average wait and turnaround times. It also prints final system statistics once all tasks have completed.

Shared Data and Synchronization

The primary shared data structure is the Stats struct, which contains all runtime metrics including:

total completed tasks
CPU and IO completion counts
busy time and total time
average wait time
average turnaround time

This shared state is wrapped in:

Arc<Mutex<Stats>>

The Arc allows multiple threads (manager, workers, and monitor) to share ownership of the same statistics object, while the Mutex ensures only one thread can modify the statistics at a time.

This protection is necessary because multiple worker threads may attempt to update counters simultaneously. Without a mutex, race conditions could occur, leading to incorrect or inconsistent statistics due to overlapping writes.

The manager also uses shared state to update global time, while the monitor reads from it to display system performance. This shared visibility ensures consistency across the entire simulation.

2. Experiment Summary (Workload Behavior and Scheduling Results)

Two experiments were conducted to evaluate system behavior under different workload distributions using a FIFO (First-Come, First-Served) scheduling policy.

Experiment A: Balanced Workload (70% IO / 30% CPU)

In this configuration, IO-bound tasks dominate the workload. Because IO tasks have lower CPU cost and occur more frequently, they are dispatched more often and complete faster. This leads to higher CPU utilization efficiency since workers are frequently able to process lightweight tasks.

The FIFO policy makes task execution easy to follow, as tasks complete in the order they arrive. However, IO tasks tend to dominate early execution cycles, while CPU tasks appear less frequently but still complete fairly over time.

Overall, this configuration produces:

higher throughput efficiency
more stable worker utilization
predictable task ordering
Experiment B: CPU-Heavy Workload (40% IO / 60% CPU)

In this configuration, CPU-bound tasks dominate the system. Since CPU tasks have higher execution cost (35 vs 10), fewer tasks can be effectively processed per scheduling window.

As a result, the system takes longer to complete all tasks and shows lower CPU efficiency compared to Experiment A. FIFO scheduling also causes CPU tasks to cluster toward later execution times, since IO tasks are more frequent and occupy earlier queue positions.

This configuration highlights:

longer overall completion time
reduced scheduling efficiency
less smooth workload distribution
Comparison Insight

The comparison shows that FIFO scheduling behaves well under balanced or IO-heavy workloads, but becomes less efficient when CPU-heavy tasks dominate. Since FIFO does not prioritize shorter jobs or distinguish task urgency, longer CPU tasks can delay overall progress.

A more advanced scheduling policy such as Shortest Job First or Round Robin would likely improve fairness and reduce average wait time in CPU-heavy scenarios.


Tool Use Disclosure

I used AI-based tools (ChatGPT) as a reference assistant while designing and debugging the concurrent scheduler project.


I used ChatGPT as a programming and systems design assistant to help with:

-debugging synchronization and channel-related issues
-structuring the final report and documentation
-summarizing my own report design and experiments so I could copy into the README
-clarifying how cross-thread communication should be structured
-explaining race conditions and synchronization problems

Advice Accepted

I accepted the recommendation to move task start-time recording from the manager thread into the worker thread. This fixed incorrect wait time calculations because tasks were previously marked as “started” before actual execution began.

Example of Advice I Rejected or Had to Modify

I did attempt a SJF policy but I couldn't get it to function correctly so I had to reject that idea. I also beleive that FIFO is better for the specifics of this project. 