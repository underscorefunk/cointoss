Coin Toss
=========

The objective of this repo is to explore state machine patterns by looking at concrete machines.

Struct states takes a simple approach to only using structs and enums for pattern matching to 
store states. Creating a new state struct yields a new state wrapped with its capabilities 
wrapped up inside it.



Accept any event out of possible events
Exterior is an IO boundary
Observe Context
Observe Current State
Receive Messages
Need event queue/inbox even for unsupported events

Machine ( state, context )

	All of the following get (context, state)

	Guards - get context and events

	Action (non-blocking) 	 - Sync is blocking
						 - Async gets forgotten

	Invocation (blocking)  	- Calling an Async function 	(action)
			 			- Calling a sync function 		(action)
			 			- Spawning a Machine
			 			- A provided callback (websocket events, etc)

Action something taking context + event and performing a side effect

Action: 				(context, event) => side effect
Assignment (Action):


Enum events
with InvalidEvent(string)

Event goes to event queue
Attempts to process it

