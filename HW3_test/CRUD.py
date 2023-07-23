class Student:
    def __init__(self, id, name, age):
        self.__id = id
        self.__name = name
        self.__age = age

    def setId(self, id):
        self.__id = id

    def getId(self, name):
        return self.__id
    
if __name__ == '__main__':
    student = Student(1, 'John', 20)
    print(student.getId('John'))