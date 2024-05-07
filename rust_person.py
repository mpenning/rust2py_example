class Person:
    def __init__(self, first_name, last_name, age):
        self.first_name = first_name
        self.last_name = last_name
        self.age = int(age)

    def have_birthday(self):
        self.age += 1

    def __str__(self):
        return f"<Person {self.first_name} {self.last_name} is {self.age} years old>"

    def __repr__(self):
        return self.__str__()

