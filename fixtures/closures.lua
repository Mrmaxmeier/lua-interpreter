function add (x)
    print("making add(" .. x .. ")")
    return function (y)
        return x + y
    end
end

add2 = add(2)
print("add2(5) = ")
print(add2(5))