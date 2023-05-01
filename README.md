Utilisation de bevy pour simulations physiques (n-body)
ECS is a software pattern that involves breaking your program up into Entities, Components, and Systems. Entities are unique "things" that are assigned groups of Components, which are then processed using Systems.

In other words:

* Entity - Every game object is an entity with a unique numeric identifier. An entity may be nothing more than an identifier, but it may also have some associated components.

* Component - A component is a data field that can be associated with an entity, e.g., position (a component of type Vec3 if working in 3D space), velocity (also of type Vec3), hit points (integer), mass (float), etc.

* System - A system is a chunk of code that runs once per update cycle (i.e., every “frame”) with reference to a specific collection of entities. The entities visible to a system are selected by a query, where the semantics of the query (the subset of the world’s entities it selects) is determined by the query’s type. This is where things get really interesting, because Bevy can schedule systems to run in parallel when it can guarantee the absence of race conditions from looking at the types of their queries.

cargo run --examples kepler_orbit
