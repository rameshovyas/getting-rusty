# Understanding how to format and print strings in Rust programming language.

This small writeup is for those who are looking for "how to print formatted values in Rust" as they have done in **C** using **printf** or using **format** in **Python**.

Being used to C and C++ we are always looking for similar functionalities in Rust programming language. The most common is how to display well formatted outputs, as this is obvious for a good CLI application. 

I will try to show how to use **print!** and **format!** macro to print and format outputs in Rust programming.

First of all we see how to use print! and println! to print formatted output on console.

**print!** and **println!** are macros that are used to print formatted string to the console. **println!** is used to append a line feed after the string we printed.

```
 println!("Hello");
 println!("Jodhpur");
```    
The above two statements outputs this
```
Hello
Jodhpur
```
Sience we have used **println!** each output will be in sperate lines, because println! says print to the terminal and append a new line.

There is an another similar macro **print!** that prints to the console or terminal but does not append a new line.

Let us replace println with print in above code and see what is the output.

```
 print!("Hello");
 print!("Jodhpur");
```    
The above two statements outputs this
```
HelloJodhpur
```

Both **print** and **println** accepcts multiple arguments where the first argument is a string literals containing {}, these {} braces are replaced by data in the following arguments, resulting into formatted output string to the console. The data in the following arguments can be constatnt or variables.

```
    println!("{} is a city in India", "Jodhpur")
```
In the above statement the {} braces will be replaced by "Jodhpur". See the output:

```
Jodhpur is a city in India
```
We can have as much {} as we want, the comiler will make sure that we have data for every {} in the following arguments.

```
  println!("{} is a city in {}", "Jodhpur", "India");
```
Output

```
Jodhpur is a city in India
``` 
In the above example we can see, by default the first placeholder ({}) is replaced by "Jodhpur" and the second with "India", this can be controlled that we will see in our following examples.

The above examples shows that the first argument is **format string** which is a string literal always and cannot be a variable. The compiler makes sures wheter we have provieded suitable number of arguments in the argument list to parse the forsmat string or not.

That was just an introduction, now lets us have a deep drive and see how we can use format and print macros for formatting output strings in Rust programming.

### Positional parameters
We have seen in the above examples, by default Rust comipler replaces the parameters in the order we have provided them in the parameter list, but using positional parameters we can control which parameter will be replaced for which place holder, let us see by a small example.

```
 println!("{1} is a city in {0} state of {2}", "Rajasthan", "Jodhpur", "India")
```
```
Jodhpur is a city in Rajasthan state of India
```
The numeric place holders decides here which argument from the arguments list will be replaced to a particular placeholder in the formatting string.

The iterator over the argument list advances to the next argument in the list by default and if not used properlly can cause improper behavior.

```
println!("{1} {} {0} {}", 10,2);
```
```
2 10 10 2
```
We can see that after replacing the first placeholder in the format string {1} with 2 the iterator advances to the next which is 10. This should be taken care when we mix match our format strings.

>We must use all of the parameters provided in the list otherwise compiler will throw an error, we can use the arguments multiple times but we have to use them atleast once.

```
println!("{1} is a city in state of {2}", "Rajasthan", "Jodhpur", "India");
```

```
println!("{1} is a city in state of {2}", "Rajasthan", "Jodhpur", "India");
   |              -------------------------------  ^^^^^^^^^^^ argument never used
   |              |
   |              formatting specifier missing
```

### Named placeholders
When the argument list is large enough it is very confusing to use positional placeholders and sometimes it causes wrong output. Instead of positional placeholders/parameter we can use named placeholders so that we may not get confused.

```
println!("My full name is {first_name} {last_name}", first_name="Ramesh", last_name="Vyas");
```
```
My full name is Ramesh Vyas
```
It is very much clear that the first bracket is identified with "first_name" and second with "last_name". Also while providing arguments in the list it is now mandatory to provided a name to each argument.

### Formatting outputs
- Specifying output width









