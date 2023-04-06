# Only Real People PGP

Creating a secure and trustable foundation for communication in a world increasingly dominated by chatbots and AI, by leveraging OpenPGP and real-world human connections.

## Introduction

The public internet is becoming more and more influenced by large language models (LLMs) and AI-driven chatbots. As these technologies advance, distinguishing between human and non-human users becomes increasingly challenging. In response to this, the Real People PGP project aims to create a network of verified human users who have met and established trust in person.

## Background

While technical solutions to verify human users are essential, sometimes the best approach is a combination of technology and social interaction. OpenPGP, a widely-used encryption standard, serves as the foundation of this project. By encouraging human users to meet and verify each other in the real world, we can create a more trustworthy network, ensuring that the person on the other end of the conversation is indeed human.

## Get Involved

We welcome contributions, ideas, and feedback to help us build a safer and more secure internet for everyone. Join us in our mission to create a network of verified human users and safeguard online communication against the growing presence of AI and chatbots.

## Host a verification server locally

```
git clone https://github.com/real-people-pgp/real-people-pgp.git
cd real-people-pgp
docker compose build
docker compose up -d
```

Now a signature verification server is running on port 8883.
It exposes an gRPC API defined in app/proto/poh.proto .

### Checking validity using a client program

You should provide a public PGP key and a PGP signature to check
its validity. 

```
cd app
cargo build --release
./target/release/client ::1 8883 public-key.asc signature.asc 
```

This program will only verify signatures that are done by public keys
that have been signed by a CA authority. represented in among the 
keys in the keyring keys/keyring.asc

## Current keyring

This is the current keyring:

```
$ cat keys/keyring.asc 
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZCXYhhYJKwYBBAHaRw8BAQdAa44vN5k1YfDmeTTaTR3Qm3ATkaZY+96Rpw3T
+vykHBK0MlJpY2thcmQgSGFsbGVyYsOkY2sgPHJpY2thcmQuaGFsbGVyYmFja0Bn
bWFpbC5jb20+iJkEExYKAEEWIQTVT+0BkT4hIK2ds8uwcm1Hgfl59wUCZCXYhgIb
AwUJA8JnAAULCQgHAgIiAgYVCgkICwIEFgIDAQIeBwIXgAAKCRCwcm1Hgfl59xTB
AP9NtOY7am1kt2abXh7pxDzsuHQQwxt11RCgITiAYlmXGQD8DMZ2lZ60F7w0aX4j
ZnZO6kKS75wy0XnO6Vv1HL9QKgyIdQQQFgoAHRYhBA+lwjcgRLcc2e4PQmu3CGnV
OS74BQJkKZ9jAAoJEGu3CGnVOS74qnwBAJDpPD2Wnv8SDVXiRshIFvPRMDHRKCuG
5j6sCVkZj+1nAQCFdmYtu8c83BFFDsr8p5W2+l+SgWn1u2XQxkvnSV0IArg4BGQl
2IYSCisGAQQBl1UBBQEBB0CIo41cJVSALGAbr6mdxa1bXD4fs5Gbk0Ktjpsf9U4c
IQMBCAeIfgQYFgoAJhYhBNVP7QGRPiEgrZ2zy7BybUeB+Xn3BQJkJdiGAhsMBQkD
wmcAAAoJELBybUeB+Xn3OjYA/RiOvenxZQs6ZkUV35HjmIM3XyylPUc3fIOfBUYC
eUczAQC7SRdABUOg+0P1cLNNUrJuLtfsnk7duxtdGwRMPTa8BZgzBGQoY10WCSsG
AQQB2kcPAQEHQBMdFSYbPbnibkujEmK762AcQS9cW+jt+LGpEzuLO+c1tEFPbmx5
IGh1bWFuIGtleXMgLSBSaWNrYXJkIEhhbGxlcmLDpGNrIDxvbmx5Lmh1bWFuLmtl
eXNAZ21haWwuY29tPoiZBBMWCgBBFiEED6XCNyBEtxzZ7g9Ca7cIadU5LvgFAmQo
Y10CGwMFCQPCZwAFCwkIBwICIgIGFQoJCAsCBBYCAwECHgcCF4AACgkQa7cIadU5
LvgywQD7BDMDFexIofT8MMedCchrDx59XnJaxOSSe+bFCUuZ+mcA/35dvD5WAcxu
Dh9t9y+P3tgtTc+K/rpCjiSVE+QP2jIMuDgEZChjXRIKKwYBBAGXVQEFAQEHQEwj
4AOM0O0zc3sN7ziAGR002fUOQNNTCQMyIJz2nr9mAwEIB4h+BBgWCgAmFiEED6XC
NyBEtxzZ7g9Ca7cIadU5LvgFAmQoY10CGwwFCQPCZwAACgkQa7cIadU5LvjikAEA
0zf7/5O5GSbe55VRwpxzbgFXonnD306u2I61K7AQI3IBAP50WXnsLx/FLXVM2Ftu
bsa7EUdfuqn4lq7yB3vX310I
=UHXv
-----END PGP PUBLIC KEY BLOCK-----
```

# A work in progress

.. with lots of things to do.
Summary this far:

* CA authority
  * The CA authority signs your PGP key, and gives you a signed key back
  * Do you want to be a CA? Let us know.
  * All CA issuers are provided with name that is trackable in this project.
* There is a simple server and client program available
  * The idea is to make it possible to host multiple of these servers
  * The server checks signature validity
* Clients use a list of domain names for hosts that is provided here, but localhost should work if the server is running and you have the latest version on it (which might be hard to guarantee, but things move slow).

# Todo

* Domain names list and signature
* TLS over the middleware
* Change so that this is based on OpenPGP instead
* Domain hosting, domain list with signatures


