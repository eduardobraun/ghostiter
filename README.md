# Witers
Witers stand for **W**indow**iter**ator**s**.

This crate is just a proof of concept.

The idea is to have iterators in Rust with border strategies (mirror, cyclic, constant...).
This is useful when solving equations like the [Advection Equation](https://en.wikipedia.org/wiki/Advection#The_advection_equation) or other EDPs and discrete math problems.

## Constant
Elements outside of the vector are mapped to a constant.
Given a vector `v = [0, 1, 2, 3, ..., N]` and `C=0`, `v[-1]=0, v[-2]=0 ...` and `v[N+1]=0, v[N+2]=0`.

## Cyclic
Wraps around the limits of the array.
Given a vector `v = [0, 1, 2, 3, ..., N]`, `v[-1]=v[N], v[-2]=v[N-1] ...` and `v[N+1]=v[0], v[N+2]=v[1]`.

## Mirror
The "outside" of the vector mirrors its inner elements.
Given a vector `v = [0, 1, 2, 3, ..., N]`, `v[-1]=v[1], v[-2]=v[2] ...` and `v[N+1]=v[N-1], v[N+2]=v[N-2]`.
