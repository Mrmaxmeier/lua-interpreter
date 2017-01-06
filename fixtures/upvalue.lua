local list = {}

local function get_item_or_else(index, or_else)
    if list[index] ~= nil then
        return list[index]
    end

    return or_else
end

list[0] = "first"
list[3] = "third"
print(get_item_or_else(0, 42))
print(get_item_or_else(1, nil))
print(get_item_or_else(1, "or_else"))
print(get_item_or_else(3, "drei"))
