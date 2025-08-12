# Actix REST Server
A small server to test basic functionality

## Content
* [General info](#general-info)
* [Demonstration](#demonstration)
* [Technologies](#technologies)
* [Plans](#plans)
* [Setup](#setup)
---

## General info
In order to familiarise myself with backend writing, encryption, administration and Rust in general - this mini project was made.

---

## Demonstration
I will now try to show you some of the functionality at the moment!

1) For example, we can generate a token by entering our email address and password!
 
![ActixRESTServer](./photo/photo1.png)

2) With the necessary role, namely Admin or Manager, we can obtain a list of all users if we have previously generated a token.

![ActixRESTServer](./photo/photo2.png)

3) With the token, we can retrieve our user's name!
 
![ActixRESTServer](./photo/photo3.png)

4) To check the functionality of our server, we can receive a ‘Hello!’ message from it.
  
![ActixRESTServer](./photo/photo4.png)

5) If we want, we can get a more personalised greeting!
 
![ActixRESTServer](./photo/photo5.png)

6) We can also register a user - at the end we will get their hash!

![ActixRESTServer](./photo/photo6.png)

7) If we want to see the current Bitcoin exchange rate against certain currencies, we can do so as follows:

![ActixRESTServer](./photo/photo7.png)

8) We can also create a wallet and retrieve its address:

![ActixRESTServer](./photo/photo8.png)

9) We can also generate a list of all products and their quantities:

[ActixRESTServer](./photo/photo9.png)

---

## Technologies
Made a project using:
* Rust
* Actix-web
* Serde
* Validator
* argon2
* jsonwebtoken
* bdk
* reqwest
  
---

## Plans

✅ User Registration

✅ User Authentication

✅ User Information Route

✅ User Roles

✅ Splitting Code into Modules

✅ Working with Application State

✅ Creating a wallet

✅ Course check

❌ Database

❌ API documentation 

❌ Deploy

❌ CORS

❌ Testing

---

## Setup
Either download the latest release or:
```
git clone https://github.com/Andezion/ActixRESTServer.git
cd ActixRESTServer
```
