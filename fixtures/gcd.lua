function recursive_gcd(a, b)
  if b == 0 then
    return a
  else
    return recursive_gcd(b, a % b)
  end
end

function gcd(m, n)
    while m ~= 0 do
        m, n = n % m, m
    end

    return n
end

print("recursive_gcd(99, 56) = " .. recursive_gcd(99, 56))
print("gcd(123, 456) = " .. gcd(123, 456))