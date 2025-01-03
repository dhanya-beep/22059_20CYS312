 firstTwoElements :: [a] -> [a]; firstTwoElements [] = []; firstTwoElements [x] = [x]; firstTwoElements (x:y:_) = [x, y]; 
firstTwoElements [1, 2, 3] 