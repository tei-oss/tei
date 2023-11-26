%% what kind of string though, UUID/ULID? %%
To loosen up all user-facing contracts all identifiers must be represented as string. Strings may be left as-is, but other data types must be encoded using [Base64URL](https://datatracker.ietf.org/doc/html/rfc4648#section-5).

