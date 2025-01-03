Prelude> firstElement :: Show a => [a] -> String; firstElement []       = "Empty list"; firstElement (x : _)  = "First element is " ++ show x;
Prelude> firstElement [1, 2, 3]

