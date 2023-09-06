# Basic-Ecommerce

I made this project to practice my Full-Stack skills.



### **Important!**

This project needs a .env file in the db folder to work. In this .env file you must have something like this:

- DATABASE_URL=postgresql://user:password@127.0.0.1:5432/products
- PORT=8000
- SERVER_URL=0.0.0.0

### SERVER API ENDPOINTS

**Products**:

| METHOD | URL | Description |
| GET | /products | get all the products |
| GET | /products/id/{id} | get product by id |
| GET | /products/category/{category} | get product by category |
| GET | /products/name/{name} | get product by name |
| POST | /products/create | create new product |
| PUT | /products/modify/id/{} | modify product by id |
| DELETE | /products/delete/id/{id} | delete product by id |

**Users**:

| GET | /users/myuser/ | get my user using user's token |
| POST | /users/create/ | create new user |
| PUT | /users/modify/id/{id} | modify user by id |
| DELETE | /users/delete/id/{id} | Show file differences that haven't been staged |

**Orders**:

| GET | /orders/userid/{userid} | get orders from user |
| GET | /orders/id/{id} | get particular order |
| POST | /orders/create/ | create new order |
| DELETE | /orders/delete/userid/{id} | Show file differences that haven't been staged |

### Features:

- Product Store
- User Store
- User auth
- Admin Dashboard for product managment

### Tech:
- Docker
- PostgreSQL
- Redis
- Rust (Actix web)
- React (Tailwind)

### Diagram of the App:

Backend -> Server Api and Database
FrontEnd -> React App which calls server API for data.

### TODOS

- Error managment for products controllers.