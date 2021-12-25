function [outw, outx, outy, outz] = block(a, A, B, C, ~, x, y, z)
    w = a; %line 1
    x = x * 0; % line 2
    x = x + z; % line 3
    x = mod(x, 26); % line 4
    
    z = z / A; % line 5
    x = x + B; % line 6
    
    x = 2*heaviside(-abs(x - w));
    
    x = 2*heaviside(-abs(x - 0));
    
    y = y * 0; % line 9
    y = y + 25; % line 10
    y = y * x; % line 11
    y = y + 1; % line 12
    z = z * y; % line 13
    outw = z;
    y = y * 0; % line 14
    y = y + w; % line 15
    y = y + C; % line 16
    y = y * x; % line 17
    z = z + y; % line 18
    
    %outw = w;
    outx = x;
    outy = y;
    outz = z;
    
    