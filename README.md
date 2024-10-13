Backend:
`www` user: 400 (user read only) privacy: cargo run
docker + Linux based (ubuntu)
Linux: CLab, 4vcpu + 4G

Frontend:
User explorer could get all codes.
No privacy data exposed.
Computation and render.

UI:
render tailwind(+react)

Testing:
4weeks 3weeks(Development) 2-3week(Test+Deploy)
Integrated test suite: client.

Documentation:
In source code.

API design paradigm:
E.g. `Option<&T>` is suggested, `&Option<T>`is avoided. No `get_` naming.
