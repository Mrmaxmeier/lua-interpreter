local list = {}

local function fib(n)
    if n <= 1 then
        return 1
    end

    if list[n] ~= nil then
        return list[n]
    end

    local res = fib(n - 1) + fib(n - 2)
    list[n] = res
    return res
end

for i = 0, 15 do
    print(fib(i));
end