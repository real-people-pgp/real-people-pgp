# Only Real People PGP

Naive attempt to create a secure and trustable foundation for communication in a world increasingly dominated by chatbots and AI, by leveraging OpenPGP and real-world human connections.

## Introduction

The public internet is becoming more and more influenced by large language models (LLMs) and AI-driven chatbots. As these technologies advance, distinguishing between human and non-human users becomes impossible unless we do something. In response to this, the Real People PGP project aims to create a network of verified human users who have met and established trust in person.

## Background

While technical solutions to verify human users are essential, we think that at present, the best approach is a combination of technology and social interaction. OpenPGP, a widely-used encryption standard, serves as the foundation of this project. By encouraging human users to meet and verify each other in the real world, we can create a more trustworthy network, ensuring that the person on the other end of the conversation is indeed human.

## Creating OpenPGP keys with GPG

GPG is a terminal command line interface program. As far as we know, there are no user friendlier alternatives.
Don't be afraid of the terminal. It is very useful!
Install GPG on your system. I will describe how this is done in Linux and MacOSX systems below.
Please consult Google or ChatGPT++ if you have more questions. 

### Installing GPG on a Mac 

You need to install homebrew. Open a terminal:

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Now install GPG:

```
brew install gnupg
```

### Install GPG on Linux (on a Debian distribution)

```
sudo apt install -y gnupg
```

## Using GPG

In case you aren't familiar with OpenPGP, here is a collection of commands.
Here are some basic commands you can use to create your PGP key using GPG (GNU Privacy Guard).

```bash
# Create a key, 4096 key length is adviced
# Make it valid at least a year.
gpg --full-generate-key
# List your keys, find its ID 
gpg --list-keys
# Export your public key in ascii
gpg --export --armor ID
# Import another key
gpg --import key
# Import this projects keys
gpg --import keys/keyring.asc
# Get key fingerprint
gpg --fingerprint ID
# List signatures a key
gpg --list-sigs ID
# Check signatures of a key
gpg --check-sigs ID
# Sign a file, create a file signature with key ID
gpg --local-user ID -a --sign FILENAME
# Verify file signature
gpg --verify FILENAME
```

In case you trust this project, maybe a person in real life told you about it or offered to sign your keys.
It might be OK to elevate the trust level of the key you got from the person or from this entire keyring.

Keys from individuals who are in line with this project can be found under the [keys/poh_pgps](https://github.com/real-people-pgp/real-people-pgp/tree/main/keys/poh_pgps) folder in the project.
The name of the file in there, should make it obvious to you who it belongs to.

Elevate trust levels like this:

```bash
gpg --edit-key ID
# At the GPG prompt, type the following command to set the trust level to ultimate:
trust
```

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

After having launched the verification server as described in the previous section
you can now check that file signatures are signed by keys from who we believe to be people.
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

mQINBGRCRjABEACvLCPqOPwuY6MgWE7wnnFU1ryaZ6N3t2REZDJ/HpqiK6A5242O
otL7iZJIRuwxa/s9nKYWmiyWkoXhsfsQPoxFhgwSSyl0TTwEpCHyykYE9isc3vhX
9I5Ro5EiM+KRnDtoEAPZ5Be0lx0VOxNbKY7CU9ePz4X09awkPCIJtLNV9lRXHvoy
eTjGKbzxkbTrPGVSZCsrfqlRxfAF+ctYkjwnplx2ZKYRjL63mFIogd9zTJM/57fE
EoCv+VHjVm6GAE0cN6AuW4EQXXtT+xuzyerP8Xficux4nqHEuZX1XMdPc3OcqieV
3YatPSjs2viHJmKNQVAidBDSaPDGPDveTUCdYuGISnxvk2gTh1q3qdgg1+MM25um
Sx93vprK8UyvVEGqZCDjYJoXQuha24XFi1nZ1sQBq6nrWbnjs5dSWDNq46+wda1g
KxAmZADHuK2AfqEKdd/bizTaTCSzi2HIR+bsGxyR3CEaGuBfR9/wVM4LUieueK/x
UN38dV/4QTGGdsISFHByx5AVWh1T204RjJ+yxapmZp2sZC7UjEiAwXhwvKCT1oSb
iY84I1CbS9LUF343RpDOhMjUNOwTG3Rlym/WP/cPO1b8/yGhx3w+Ss123nb3pKxR
72lXpdgD4ZRov0+wOLoxDwsoJBw4ZBDVaxfAkkwCo9imYqL0cp7Vv8SxLwARAQAB
tJBSaWNrYXJkIEhhbGxlcmLDpGNrIChJIHdhcyBib3JuIGluIDE5OTMgaW4gVsOk
cm1sYW5kLCBTd2VkZW4uIEkgb25seSBzaWduIGtleXMgZnJvbSBwZW9wbGUgSSBo
YXZlIG1ldCBpbiByZWFsIGxpZmUuKSA8b25seS5odW1hbi5rZXlzQGdtYWlsLmNv
bT6JAlcEEwEIAEEWIQQCMssihiY4nxRIMjwqFwJRQ4+h0QUCZEJGMAIbAwUJCWYB
gAULCQgHAgIiAgYVCgkICwIEFgIDAQIeBwIXgAAKCRAqFwJRQ4+h0Wa2EACQ8wHH
g4gU+1uNZN3CS0NUzjYI0n0uveDsw7v4rUNSa4tsUI87uOXycMcmT2/oo9J1QuD/
6a6NOuGaDMoE+Qa/wNY4A3QFfznbTznU3FIGQ5BBR7wE7fb88EXAmFfSuKXUC8GG
Cpvnh8g6FDCJ5vOV/dZxvOvDIGtuQc26qgVhT4vZnwYRMaON5LHP85VIEVnYTWk8
hr2YW9CxpDN3psYKi3av1LtTtC6ybOFtTUVuBNEmX7OFLCG3eBFbTPGQ1HofN6n0
ZL/Omt/ZjN5HYysEZrIKts7rBRe76O5/tabY933y4WKAu5rG+eotHKMpk8aRvies
a6nyDJVty7CpNW07tf/XQeVPNlwzjbBiASNJe5hwlJ1im6MKmNBYbM0E9raOTTYu
2KinXlijse9KLBVuYmLazR/PPcev4e+YtIHoOd6L4EMO2emhqMyzdDRMI7JOO4p1
rFXpiVurLx5SVVxQ9AdcXuFzs83EAYPmwdyrAX68pGCuaNMmAnBK8y4F2uCJkaxZ
tKyoj+CqHCEUSVqvwoZ/2fAuND81KBp7DhYsWJngYty2TVH739ro7n+FuT2Gbp/7
0/GXdN5rCeVN/ZS8ymNJlgvGkKG6oAE9so1wDyDUUbeLFpUkUsWZBQxo2JR/dc6u
QNCQ9axUIeILSbJogLi9WLsxA/7NTdmw+dkpJLkCDQRkQkYwARAA5k2w2FmucGH5
X8/blAG6VdaxQuAamLtlZXuIP109lX/CfSTFDeh1PJeCnG4yrKyPo2gfEIidHfP6
8BV/uhXIitkg1E5ih9LCwngYYVkC4PlVC4wBmInDFmmDIR5K8Xhyh0j6C6VEWEfe
2MtZIoZFDqq3xV57mtg5u7wdOfBS4VGwolM1eW9NgiZGu22sJ4MQefGc05fI6WMn
DfdDQBdWu838ROr6/32JGKOdkLB1AuafDwSMQ65EiN/U1EveL5iut/1RFKoaIlfb
1VxDM/8RuKMKOtTFMEX//T7lufEjAdqL3s2/GF5Dx3dnsTnvJzKuKf+yr0WXLBCS
iKDQYMnYYVLaXdp0t4FHoa7x0UkGKUfx/Q+ajGVuObX3he5seURfcWMvJDov7PmR
ZnF9rc2eu2oXrjtCVlBDrrYUA5DCyw6UEdtnhVYkR9HTzrkptv3udUKMwpz8bfJe
mVwdL5BhjSsQEkl0lIfmV2FIgLpJkLLAJJFcK6hXwlTKEMBvrgtZsXGp5Y8QjIVi
8EqPyyj4LVLWsBnkOlckR8nUw9R7O4GHKhd+vL5dfkz0mS6EpT5403TcxIW0ljDM
nYhYjdOnaC2GJOCQI4DC+bbIl/cumSOfmGjPiGsFH9mcBo+BOICb3SvuOZNDbqDS
CQoBQA6FR9Np1v9auelkesixlyhdEpcAEQEAAYkCPAQYAQgAJhYhBAIyyyKGJjif
FEgyPCoXAlFDj6HRBQJkQkYwAhsMBQkJZgGAAAoJECoXAlFDj6HRYe0P/3WXVI/+
+/u4MRhY2zHT11DQ1f31w6sqGLYMZeKbbU9IHIf8TbWEFCscolZDGKVSK4Qc40bg
FfnVsZwFYHuTwYDTZZLeY/qXJbxIYXCVLaAolDvt7SkakKCt+gILTc4LvHkPNjse
vHgqeoa5lGwCzXS1V0o8h6oKN864+LL00N3VUh3Hj70VfX88pG3KUcK9lNcxHc3j
lRwJWS3gqDGoJ5LymvHXRxAQShzish5JcOdAm8zpo6gWVesJcXVnenkbd1BFxy5a
jz2rO3D3V3R8L5JqqBHiO0luAePeDaA5X7gxyznwgzdNa1FTZ2RTvpzWxP3T9NVr
9legYoiF1mzVrqrtAb88RdA6IfQxDD1glXSt5v32JlNAZX39m3uTGrnOjo3gZWif
n+vjpMR9y5wKUGjm1oxbses95NZuZv9xxmyS8UJjSFu0iopOkUzdc9in7jt5H6mk
D2SFn16aQ+uyt563boZ9GKeUXKFpCsv0ynAjgv6usBFdnQtiIYdOhfMWcGRA+12b
gNVxZdg8AEdX7tZ7WbwzBADaFcyheCMWPia7SOV348pEmdNDEI0jxAFnJEuwMdGV
buyds19HjJRB+il86Of3nJFzQ0XYNnJs9PEGuJRGlkmoiNY2FYAOfkfcM6Tpu2Ik
7kJWo0A2ef8Uo0ktgPklaBt6hQbw+7eMN7+2
=GS2/
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


