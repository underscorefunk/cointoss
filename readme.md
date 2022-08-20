Coin Toss
=========

## Threading requirements
Invocations in machines require us to spawn threads, creating actors and using message passing 
as communication. Machines that invoke other machines will have their threads blocked while the 
child (invoked) machines process their invocations.

Machine
1. enter state
2. spawn invocation
   1. thread paused while processing
   2. thread joined when complete
3. end of enter state

This is problematic because we need the machine to continue to accept messages.

The solution is to spawn each machine on a thread which is always looking for messages and 
is responding to messages. These messages follow the query command pattern with queries 
returning values but not changing the machine and commands changing the machine but not 
yielding a response.
    
    query
        state
        context
        last_action
    command
        action

The above messsges are THE ONLY way we can interact with the machine!

TODO: Add support for reaching into the graph to send messages down or walk up to retrieve 
the graph of nested machines

Machine
    State (only inner mutability)
    Context (only inner mutability)

Reactive ui works as a function of pulling updates (data + address) off the event queue
The updated state/context event queue is populated by pushing updates to the updated s/c queue.

Machine struct has a cache state (reference to it) and a send method that pushes things done 
the channel. 

Integrate with YEW

This is the wrong solution and should be done with async