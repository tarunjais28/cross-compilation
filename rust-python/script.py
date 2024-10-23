import sys  

def greet(name):
    return f"Hello, {name}!"

print(sys.argv.__len__())
iter = sys.argv.__iter__()
for val in iter:
    print(val)

print(greet(sys.argv[1]))