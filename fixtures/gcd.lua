function recursive_gcd(a, b)
  if b == 0 then
    return a
  else
    return recursive_gcd.gcd(b, a % b)
  end
end

function gcd(m, n)
    while m ~= 0 do
        m, n = math.mod(n, m), m
    end

    return n
end