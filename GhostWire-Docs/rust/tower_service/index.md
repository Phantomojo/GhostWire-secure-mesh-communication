Crate tower\_service Copy item path

<span class="sub-heading"><a href="../src/tower_service/lib.rs.html#1-390" class="src">Source</a>
</span>

Expand description

Definition of the core `Service` trait to Tower

The [`Service`](trait.Service.html "trait tower_service::Service") trait
provides the necessary abstractions for defining request / response
clients and servers. It is simple but powerful and is used as the
foundation for the rest of Tower.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Service.html" class="trait"
title="trait tower_service::Service">Service</a>  
An asynchronous function from a `Request` to a `Response`.
