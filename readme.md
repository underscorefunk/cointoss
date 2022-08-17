Coin Toss
=========

The objective of this repo is to explore state machine patterns by looking at concrete machines.

## Invocations
- Invocations are spawned when they enter their invoked state.
- Invocations are cleaned up when they leave their invoked state.
- Invocations require a channel to send their events(data) back to their parent

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

