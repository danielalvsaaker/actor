# actor
Learning project to demonstrate two different actor implementations.

**actor_dynamic**
- Traits for handling different messages.
- Trait object under the hood for dispatching messages dynamically to an actor.

**actor_static**
- Logic for handling messages must be implemented manually in a single handler function.
- Static dispatch, where an actor can only handle a single type.
- Can use an enum to emulate handling different messages.
