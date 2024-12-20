type Student = (String, Int, [Int])

averageMarks :: Student -> Double
averageMarks (_, _, marks) 
    | null marks = 0.0
    | otherwise = fromIntegral (sum marks) / fromIntegral (length marks)

displayStudentAverages :: [Student] -> IO ()
displayStudentAverages students = 
    mapM_ printStudentAverage students
  where
    printStudentAverage (name, _, marks) = 
        putStrLn $ name ++ " Average: " ++ 
                   show (averageMarks (name, 0, marks))

