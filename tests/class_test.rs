use smashlang::{execute, Value};

#[test]
fn test_class_declaration() {
    let code = r#"
        class Person {
            constructor(name, age) {
                this.name = name;
                this.age = age;
            }
            
            greet() {
                return `Hello, my name is ${this.name} and I am ${this.age} years old.`;
            }
        }
        
        const person = new Person("John", 30);
        person.name;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "John");
}

#[test]
fn test_class_methods() {
    let code = r#"
        class Calculator {
            add(a, b) {
                return a + b;
            }
            
            subtract(a, b) {
                return a - b;
            }
            
            multiply(a, b) {
                return a * b;
            }
            
            divide(a, b) {
                if (b === 0) {
                    throw new Error("Division by zero");
                }
                return a / b;
            }
        }
        
        const calc = new Calculator();
        calc.add(5, 3);
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_class_inheritance() {
    let code = r#"
        class Animal {
            constructor(name) {
                this.name = name;
            }
            
            speak() {
                return `${this.name} makes a noise.`;
            }
        }
        
        class Dog extends Animal {
            constructor(name, breed) {
                super(name);
                this.breed = breed;
            }
            
            speak() {
                return `${this.name} barks.`;
            }
            
            getBreed() {
                return this.breed;
            }
        }
        
        const dog = new Dog("Rex", "German Shepherd");
        dog.speak();
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Rex barks.");
}

#[test]
fn test_static_methods_and_properties() {
    let code = r#"
        class MathUtils {
            static PI = 3.14159;
            
            static square(x) {
                return x * x;
            }
            
            static cube(x) {
                return x * x * x;
            }
        }
        
        MathUtils.square(4);
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "16");
}

#[test]
fn test_private_fields_and_methods() {
    let code = r#"
        class Counter {
            #count = 0;
            
            #increment() {
                this.#count++;
            }
            
            increment() {
                this.#increment();
            }
            
            getCount() {
                return this.#count;
            }
        }
        
        const counter = new Counter();
        counter.increment();
        counter.increment();
        counter.getCount();
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_class_this_binding() {
    let code = r#"
        class Person {
            constructor(name) {
                this.name = name;
            }
            
            getName() {
                return this.name;
            }
            
            getNameArrow = () => {
                return this.name;
            }
        }
        
        const person = new Person("John");
        const getName = person.getName;
        const getNameArrow = person.getNameArrow;
        
        // Regular method loses 'this' binding
        try {
            getName();
        } catch (e) {
            "Error: " + e.message;
        }
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Error: Cannot read property 'name' of undefined");
}

#[test]
fn test_class_property_initializers() {
    let code = r#"
        class Product {
            name = "Default Product";
            price = 0;
            
            constructor(name, price) {
                if (name) this.name = name;
                if (price) this.price = price;
            }
            
            getInfo() {
                return `${this.name}: $${this.price}`;
            }
        }
        
        const product1 = new Product();
        const product2 = new Product("Phone", 999);
        
        product1.getInfo() + " | " + product2.getInfo();
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Default Product: $0 | Phone: $999");
}