use std::{thread, time::Duration};


fn simple_thread(){

    thread::spawn(|| {

        for i in 1..10{
            println!("first thread count{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("MAIN thread count{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    //when the main thread done, the first thread stops to
    
}

fn thread_with_join(){
    //thread.spawn retruns a JoinHandle<()>, we can store and use it to make main thread wait for the first thread to finish its process
    let handle = thread::spawn(|| {

        for i in 1..10{
            println!("first thread count{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("MAIN thread count{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //wait here for first thread to finish its job!
    handle.join();

}

fn thread_with_variable(){

    let v = vec![1,2,3];
    //for the next thread defining we get an error because, 
    //Rust cannot know how long a thread will run and whether the v variable will alive during that time. 
    /* 
    let handle = thread::spawn(|| {
        println!("The vector is : {:?}", v);
    });
    */

    //we can fix the issue by forcing the thread function to get ownership of the variable by using 'move' keyword
     let handle = thread::spawn(move || {
        println!("The vector is : {:?}", v);
    });
}

fn thread_with_channel(){
    use std::sync::mpsc;

    //the channel func retruns a tuple (sender, receiver)
    let (tx,rx) = mpsc::channel();

    
    thread::spawn(move || {
        let msg = String::from("hi");
        //send() func retruns a Resul Enum, we need to handle if an error retruns. For now, is error the rust will panic with unwrap()
        tx.send(msg).unwrap()

        //send func get the owner ship of the msg, so we cant modify or drop the msg after send it 
        
    });

    let received_message = rx.recv().unwrap(); 
    //recv() blocks the main thread and wait until returns data or error, then retruns result enum
    //try_recv() does not block the main thread. 
     //it looks at the channel and immediatly retruns a result enum, if there is data it retruns it or returns error
    println!("The Received Message: {}", received_message);
}

fn iterating_over_channel(){
    use std::sync::mpsc;

    let (tx,rx) = mpsc::channel();

    thread::spawn(move ||{
        let values = vec![1,2,3,4,5];

        for val in values{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //we can iterate over rx. it loop till the channel gets close  
    for received in rx{
        println!("The Received Value: {}", received);
    }
}

fn multiple_sender_threads(){
    use std::sync::mpsc;

    //rx can only listen by one thread!!
    let (tx,rx) = mpsc::channel();

    //we cant use the same channel in two different sender threads
    //we have to create a clone of the channel for the second sender thread
    let tx2 = tx.clone();

    //first sender thread which use original channel
    thread::spawn(move ||{
        let values = vec![1,2,3,4,5];

        for val in values{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

     //second sender thread which use clone channel
    thread::spawn(move ||{
        //the first value which sended over tx defines the type of the channel,
        //So we cant send onther type of data by another thread
        let values = vec![10,20,30,40,50];

        for val in values{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //we can iterate over rx. it loop till the channel gets close  
    for received in rx{
        println!("The Received Value: {}", received);
    }
}

fn mutex(){
   //Mutex allows access to data to only one thread at a time, thus preventing data races.
   //Data (T) is stored in a Mutex<T>. To access this data, a lock is first acquired. If the lock() call is successful, 
        //a structure called MutexGuard is returned.
        //As long as this structure is active, no other thread can access the data. 
    
    use std::sync::Mutex;

    let my_mutexed_value = Mutex::new(5);

    println!("Before Manuplating The Mutexed Value Is :{:?}", my_mutexed_value);

    {
        //lock() func retruns a Result Enum so we can use unwrap() for now
        let mut  the_value = my_mutexed_value.lock().unwrap();
        // If no other thread has occurred a lock for my_mutexed_value, 
        
        // we have to deref the smart pointer to manuplate the value
        *the_value = 15;
      
    }

     //the lock for the my_mutexed_value is acquired here and automatically released when the scope ends, so that it is not forgotten to be released.

    println!("After Manuplating The Mutexed Value Is :{:?}", my_mutexed_value);

    //if the poisoned value = false, the data is safe to use,
    // if it is True, it means some other threads tried to get lock and paniced when a thread has the lock, so be careful when using data

}

fn atomic_counter(){
    //Atomic Counter is a counter that can be safely updated by multiple threads simultaneously.
    //Mutex can couse deadlocks! 

    use std::sync::{Arc, Mutex};
    use std::thread;

    //creating an Arc with a Mutex, to make it safe
    //Mutex uses interior mutebility so we dont have to define the couter as mut
    let counter = Arc::new(Mutex::new(0));

    //creating an empty vector to hold the all threads
    let mut handles = vec![];

    for _ in 0..5{
        //creating a clone of the counter reference to be able to use it by multiple threads 
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move ||{
            //each thread will take the lock of the counter and increase it 1 then release the lock.
            let mut num = counter.lock().unwrap();
            *num +=1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    //to be able to read the mutex value we need to get the lock to
    println!("Counter counts to: {}", *counter.lock().unwrap());

}


fn main(){

    //simple_thread();
    //thread_with_join();
    //thread_with_channel();
    //iterating_over_channel();
    //multiple_sender_threads();
    //mutex();
    atomic_counter();


}