---
id: EXP-001
title: Structural Ontologies for a Multi-Graph Space
status: exploring
created: 2026-10-30
---

# EXP-001: Structural Ontologies for a Multi-Graph Space

## 1. Starting Point

Property graphs used with AI systems are, in practice, collections of graphs that differ in kind and formality: unstructured and structured data, many domains, and scales from millions of small memory graphs to monolithic enterprise graphs, with external data sources projected in and viewed as graphs. That is already a multi-graph space, by convention. What would it mean to formalize it so that it stays modular and composable, with local variance and global coherence?

The stance taken here is that form precedes function: structure comes before semantics. It starts from one observation. The power of a graph model is that relationships are externalized from nodes. That is natural in the mathematics, where nodes do not know about relationships, and awkward in an object-oriented approach. The payoff is easy extensibility, adaptability, and room for several simultaneous viewpoints.

The aim is to describe formally a graph of graphs, with both graph-level and element-level relationships.

## 2. Landscape

Ontologies usually start with strict formalism and then leave gaps for informal conventions and unknown additions, which is how they stay flexible. The multi-graph space suggests the reverse: let data exist structurally first, then classify it, and treat informal semantics (descriptions, definitions, tags) as attachable information rather than as rigid types. Entity-component-system designs work this way: an entity is a bare identity, and components attached to it carry data and meaning.

Topos theory and applied category theory offer vocabulary for composing graphs without forcing a single global schema. The framings below are drawn from a conversation with a language model (the full text is in the source appendix) and are recorded as candidate vocabulary, not as verified mathematics.

## 3. Divergent Framings

### 3.1 Colimits and pushouts: gluing graphs along an interface

Two domain graphs, say finance and HR, share a sub-graph of common identifiers, the interface graph. Maps from the interface into each domain define a pushout: the unified multi-graph that stitches the domains together along exactly that boundary, without duplicating shared data. This is the weakest and most purely structural form. It promises lossless, deterministic schema merging, and says exactly how a change in one domain propagates or stays local.

### 3.2 Monoidal categories and operads: sub-graphs as boxes with ports

A subdomain is a box with exposed ports (nodes or edges meant for external connection), and wiring boxes together is an operad algebra. Composition is associative, so composing A with B and then adding C gives the same result in any order. This is the behavioral form: subdomains behave as black boxes that can be refactored as long as their exposed interfaces hold.

### 3.3 Presheaves and topoi over preorders: local views and partial truth

A property graph is modeled as a presheaf from a small category of entity and relationship types to sets. If subdomains are ordered hierarchically or by access level, the underlying structure is a preorder, and a presheaf on it gives each subdomain a local view. The topos structure supplies a generalized truth: a relationship can be true in one subdomain, unknown in another, and globally restricted. This is the logical form, aimed at queries across subdomains and at partial views and access control.

### 3.4 Spans of graph homomorphisms: graphs as objects, a relationship graph between them

Autonomous domain graphs are the outer objects. A bridge graph, the span, contains only relationships connecting nodes of the spanned graphs, and homomorphisms from the bridge into each domain align them. Domains keep full internal variance and do not know about each other. The span holds identity resolution. A global federated view arises by composing spans with a pullback, so overlaps are computed where spans meet and domain-specific attributes ride along without causing schema conflicts. Three rules were proposed: bridge nodes are abstract identity aliases, domain-specific logic stays out of the bridge, and the bridge enforces structural integrity when a domain element is removed.

### 3.5 The ECS inversion with layered semantics

The graph is a superstructure. Nodes are anonymous identities. Properties, tags, and informal descriptions are components attached to them, and spans act as the systems that relate them across boundaries. Semantics then layers on gradually: informal annotations first (a description on a span edge saying what a loose match means), then informal types that emerge from usage, then tightened boundaries enforced where a span meets a domain, so that domains stay messy and the bridge becomes a validated checkpoint. In categorical terms, semantics is a mapping from raw structure to a category of meanings, which can be swapped or tightened without rewriting the graph.

## 4. Tensions

- **Who owns identity.** The span could hold identity resolution (the answer given in the source conversation), or domains could share an identifier layer such as an enterprise ID.
- **Strict first or structure first.** Traditional ontologies formalize first and patch in informal conventions. The structure-first framings do the opposite, and the two disagree about what a "valid" element is before any semantics is attached.
- **A semantics-free bridge versus relationships that need assertions.** Keeping the bridge free of business logic conflicts with relationships that carry real properties.
- **Where to tighten.** Constraints can sit at the span interface (3.4, 3.5) or inside each domain (the usual schema approach). These place the boundary of responsibility in different places.
- **Vocabulary versus buildability.** 3.1 to 3.3 give precise language but no obvious path to an implementation. 3.4 and 3.5 are closer to something buildable but lean on that language loosely.

## 5. Directions Worth Pursuing

The working direction combines 3.4 and 3.5: graphs as objects, spans as the relationship graphs between them, informal semantics attached as components, and boundaries tightened at the spans over time. Pushouts (3.1) are a candidate operation for composing, and the topos logic (3.3) is a candidate for partial views later. None of this commits to a formal framework.

## 6. Disposition

Candidate RFC, not yet written: what a valid gram ontology looks like in a multi-graph space, in terms of graphs, spans, and layered semantics. Nothing here is decided. A concrete elaboration of the span-and-layering framing would show what it looks like in practice.

## 7. Appendix: References

- AlgebraicJulia, and categorical databases as developed by David Spivak and collaborators: computational systems that use presheaves and colimits to migrate, merge, and split schemas.
- The Neo4j ontology-graph-spec, a JSON/YAML format for graph descriptions with `description`, `aliases`, and `reference`. It is a lower bound for what an ontology here needs to cover, and it lacks features a multi-graph space needs, such as spans.
- A version of the frame/span approach is being explored in pattern-hs.

## 8. Appendix: Source Material

The framings in section 3 and the layering strategy in 3.5 come from the following chat with a language model, kept here verbatim. Provenance: this text first appeared in `structural-ontology-notes.md` under `proposals/` in relateby/pattern-rs. Treat it as raw material, not as verified mathematics.

---

To achieve modularization and composability, you can look to three progressively stronger "weaker forms" of topos theory from Applied Category Theory (ACT).


1. Colimits and Pushouts (The Weakest, Purely Structural Form)
If you don't need a full logical universe (a topos), you might just need a way to snap graphs together like Legos. In category theory, this is handled by colimits—specifically, a pushout.
* How it works: You have Domain Graph \(A\) (e.g., Finance) and Domain Graph \(B\) (e.g., HR). They share a sub-graph of common identifiers (e.g., Employees/Contracts), which we will call Interface Graph \(I\).
* The Operation: By defining map arrows \(I \to A\) and \(I \to B\), the pushout is the unified multi-graph \(M\) that stitches \(A\) and \(B\) together exactly along the boundaries defined by \(I\), without duplicating the shared data.

   [ Interface Graph I ]
       /           \
      v             v
[ Domain A ]    [ Domain B ]
      \             /
       v           v
  [ Composed Multi-Graph M ]
* Why it's useful: It provides a mathematical guarantee for lossless, deterministic schema merging. If a property changes in a subdomain, the pushout mechanics dictate exactly how that change propagates or stays localized.

2. Monoidal Categories and Operads (The Behavioral Form)
If your production graphs aren't just static data structures but represent workflows or composable modules with "input/output ports," you are looking at a symmetric monoidal category or an operad.
* How it works: Instead of looking at a graph as a whole, a subdomain is treated as a box with exposed "ports" (nodes or edges intended for external connection).
* The Operation: The act of wiring sub-graphs together is formalized as an operad algebra. You can nest sub-graphs inside larger sub-graphs, and the category's monoidal product (\(\otimes \)) guarantees that composing Domain \(A\) with Domain \(B\), and then adding Domain \(C\), yields the same result regardless of the order of operations.
* Why it's useful: This is the foundation of modular software architecture. It allows you to build a strict API layer for property graphs, treating subdomains as black boxes that can be safely refactored as long as their exposed interfaces remain unchanged.

3. Presheaves and Topoi over Preorders (The Logical Form)
If you need to query across these subdomains while maintaining boundaries, you are looking at a Presheaf Topos (which is weaker and much easier to implement than a Grothendieck Topos on a complex site).
A property graph can be modeled as a presheaf: a functor from a small index category (representing your schema's entity/relationship types) to the category of sets.
* How it works: If your subdomains are arranged hierarchically or by access level, your underlying site is just a partial order (a preorder).
* The Operation: A presheaf on a preorder gives you a "local view" of the data. The topos structure guarantees the existence of a subobject classifier (a generalized true/false logic).
* Why it's useful: This gives you a mathematical framework for multi-graph access control and partial data views. Instead of Boolean logic (an element is either in the graph or not), the topos logic allows queries to return answers like "This relationship is true in the HR subdomain, unknown in the Engineering subdomain, and globally restricted."



Real-World Execution: Algebraic Data Types (ADTs)
If you want to move this from theory to code, look into Algebraic Julia (AlgebraicJulia) or the concept of Categorical Databases (pioneered by mathematicians like David Spivak). They have built working computational engines that use presheaves and colimits to dynamically migrate, merge, and split relational and graph data schemas deterministically.


One lesson from property graphs is that under specifying semantics while enabling structure results in an highly adaptive yet coherent system. I think (am headed in the direction) that we need: graphs as objects with “spans” as relationship graphs whose internal elements are only relationships connecting nodes from the “spanned “ graphs. This gives us a global federated view with local overlaps and internal variances


You have hit on a profoundly elegant architecture. By shifting from a paradigm of "one giant global schema"to "localized graphs linked by bridging structures," you are leaning into the core philosophy of category theory.


Your insight about under-specifying semantics while enabling structure is exactly how Category Theory operates: it doesn't care what is inside an object; it cares entirely about how that object relates to others.
What you are describing—graphs as objects with relationships between them mediated by an intervening graph of connections—maps beautifully to a specific mathematical construct in applied category theory called a Span of Graph Homomorphisms (or more generally, an Open Graph framework).
Here is how your architectural vision translates into structural mechanics.



1. The Anatomy of your "Span" Architecture
In category theory, a span is a diagram of three objects where one central object maps to two (or more) outer objects: A ← S → B.
In your proposed property graph mesh:
* The Outer Objects (\(G_1, G_2\)): These are your autonomous domain graphs (e.g., FinTech Graph and HR Graph). They have full internal variance. They use whatever properties, node labels, and edge types their respective engineering teams require. They do not know about each other.
* The Bridge Object (\(S\)): This is your relationship graph (the span). As you noted, its internal elements are strictly mapping relationships.
* The Morphisms (The Arms of the Span): These are graph homomorphisms (structural mappings) that point from the bridge graph \(S\) into \(G_{1}\) and \(G_{2}\).

          [ Relationship Graph S ]  <-- (The Span: Contains only alignment links)
                /          \
  Mapping to   /            \   Mapping to
  Left Nodes  v              v  Right Nodes
       [ Domain Graph G1 ]        [ Domain Graph G2 ]
       (e.g., FinTech)            (e.g., HR)

How it Resolves the Property Graph Dilemma
Because the bridge graph \(S\) only contains relationships connecting nodes across boundaries, you preserve absolute freedom.
* If \(G_{1}\) changes its internal properties, \(G_{2}\) doesn't break.
* The span \(S\) handles the structural alignment without forcing \(G_{1}\) and \(G_{2}\) to agree on an overarching semantic dictionary.



2. Achieving the Global Federated View
When you want to query across the entire system, how do you compute the "global federated view" without physically copying all the data into a single, massive monolithic database?
You execute a Composition of Spans via a categorical operation called a Pullback.
If you have a Span linking Domain A to Domain B (A ← S₁ → B), and another Span linking Domain B to Domain C (B ← S₂ → C), category theory provides a deterministic algorithm to automatically construct a new, combined Span linking Domain A directly to Domain C (A ← S₃ → C).

        [ Pullback S3 ]
          /         \
         v           v
      [ S1 ]       [ S2 ]
      /    \       /    \
     v      v     v      v
   [ A ]   [  B  ]     [ C ]
* Local Overlaps: The overlaps are computed lazily or on-demand exactly where the spans intersect at Domain B.
* Internal Variances: Because the pullback only cares about the matching nodes defined by the spans, the vast majority of your domain-specific attributes (the variance) ride along passively without causing schema conflicts.



3. Practical Implementation Guidelines
To turn this mathematical vision into an engineering reality for production property graphs, you can enforce three structural rules at the software layer:
* Strict Identity Spans: The node types inside the bridge graph \(S\) should be abstract "Identity Aliases" (e.g., a node in \(S\) just says Entity_X). One outgoing edge points to G1.User_123 and another points to G2.Employee_ABC.
* Zero Semantic Baggage on Bridges: Do not allow domain-specific business logic to creep into the bridge graph \(S\). If a relationship requires complex property assertions, it belongs inside a local domain graph, not in the connective tissue.
* Graph Homomorphism Constraints: Ensure that your bridging microservices enforce graph structural integrity. If a node in a domain graph (\(G_{1}\)) is deleted, the corresponding mapping node in the bridge graph (\(S\)) must automatically cascade-delete or gracefully flag a broken link, preserving the validity of the global federated view.
This gives you a system that is incredibly resilient. Subdomains can evolve at different speeds, schemas can diverge wildly, and yet the global architecture remains perfectly coherent because the bonds between the graphs are treated as first-class citizens.

As you design this, what is your primary strategy for resolving identity across the domains? For example:
* Do your domain graphs already share a common identifier layer (like an Enterprise UUID or a Master Data Management ID)?
* Or will the relationship graph (the span) be responsible for evaluating and holding the probabilistic or deterministic matches between disparate keys?


The span would be responsible. With this approach, is there a natural way to layer on semantics (informal descriptions and definitions, not formal logic) and then tighten up the boundaries? 

I often think back to the entity-component-system approach to using a graph as a superstructure to which information can be attached. This inverts the usual approach to ontologies which start with strict formalism and must leave gaps for informal conventions or unknown additions to achieve flexibility.


You are describing a profound inversion of traditional data modeling. By starting with structure first (the graph as a superstructure) and layering semantics on top, you align perfectly with how both modern game engines handle highly dynamic worlds (Entity-Component-System or ECS) and how applied category theory models complex systems.
Inverting the workflow—allowing data to exist structurally before it is classified semantically—is the exact antidote to "ontology paralysis," where systems break the moment reality diverges from a strict, pre-defined schema.
Here is how you can naturally layer informal semantics onto your span-based graph superstructure and systematically tighten the boundaries over time.



1. The ECS Inversion: Graph as the Superstructure
In a traditional ontology, a node is its type (e.g., a hardcoded Person class). In an ECS-inspired graph architecture:
* The Entity (Identity): A node in your graph is just a raw, anonymous identifier (a Guid). It has no intrinsic meaning.
* The Component (Data & Semantics): Properties, tags, informal descriptions, and definitions are decoupled components attached to that identifier.
* The System (The Span/Context): The spans act as the systems that look at these components and decide how they relate across boundaries.
By treating informal semantics as modular components rather than rigid types, you can attach a "tag" or a "markdown string definition" to any node or edge without altering the graph's physical routing topology.



2. The Layering Strategy: From "Wild West" to Tight Boundaries
Because your spans hold the identity and structural alignment, you can introduce a natural, evolutionary pipeline for semantics that moves from informal documentation to tighter boundaries.

[ Structural Span ] ---> [ Tagging & Notes ] ---> [ Local Constraints ] ---> [ Boundary Enforcement ]
(Raw Connection)         (Informal/Human)         (Subdomain Rules)          (Strict Validation)

Layer 1: Informal Annotations (The "Tags and Wiki" Layer)
Start by allowing your spans and domain graphs to carry unvalidated metadata components.
* Nodes and Edges as Anchors: A relationship edge in your span graph doesn't just link G1 and G2; it carries an informal component like description: "This link represents a loose match used by marketing, often based on shared IP addresses."
* Why it works: It allows humans and LLMs to immediately understand the intent of the connection without forcing a database administrator to write a strict schema validation rule. It keeps the system highly adaptive.

Layer 2: Typological Clustering (Emergent Semantics)
As patterns naturally emerge from how your team connects graphs via spans, you can begin grouping them into "informal types" based on usage rather than strict logic.
* If a collection of anonymous span edges consistently carries properties like currency, amount, and timestamp, they are de facto acting as a transaction ledger.
* You can assign an emergent semantic label to this cluster (e.g., #FinancialBridge) without locking down the database schema.

Layer 3: Tightening the Boundaries (The Gateway Layer)
Once an emergent pattern stabilizes and a business use case requires high data integrity, you tighten the boundaries at the span interface, not inside the subdomains.
* Contract-Driven Spans: You introduce a schema validator specifically to the arms of the span (S → G1and S → G2).
* The Rule: For a node to pass through this specific span into the global federated view, it must possess specific components (e.g., it must have a verified tax_id property).
* The Result: The internal subdomains (G1 and G2) remain completely flexible and messy ("internal variances"). However, the bridge between them becomes a strict, validated checkpoint. You get a clean, reliable global view without destroying local autonomy.



3. The Categorical View: Functors from Structure to Semantics
In category theory, this approach maps beautifully to the concept of an algebraic theory or a functorial semantics.
Instead of embedding the rules into the objects, you separate the syntax (the structural graph layout and identity spans) from the semantics (the meaning or data components).
Semantics is just a mapping (a functor) from your raw graph structure to a category of meanings (like human-readable text, dictionary definitions, or eventually, database types). If you want to change the semantics or tighten the rules, you don't rewrite the graph database; you simply swap out or tighten the mapping function.
This allows your production graph mesh to act like a living organism: it can rapidly adapt structurally to new data inputs, while your understanding and governance of that data can catch up semantically at its own pace.

As you think about implementing this ECS-style layering, how do you envision team collaboration? For instance:
* Will domain engineers be the ones adding informal descriptions to the spans as they discover connections?
* Or are you designing this so an autonomous system (like an LLM or an identity resolution engine)can programmatically stamp these semantic components onto the graph superstructure?
