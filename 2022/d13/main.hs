import Text.ParserCombinators.ReadP
import Data.Char
import Data.List
import Data.Maybe

data Packet = Node [Packet] | Leaf Int
	deriving Eq

instance Ord Packet where
	compare (Leaf l) (Leaf r) = compare l r
	compare l r@(Leaf _)      = compare l (Node [r])
	compare l@(Leaf _) r      = compare (Node [l]) r
	compare (Node l) (Node r) = compare l r

parse line = readP_to_S packet line
packet = (Node <$> between (char '[') (char ']') (sepBy packet (char ',')))
	+++ (Leaf <$> num)
num = read <$> many1 (satisfy isDigit)

comparePackets [] _ = 0
comparePackets (l:r:xs) idx = idx * (if l < r then 1 else 0) + comparePackets xs (idx + 1)

main = do
	content <- readFile "input.txt"
	let rawPackets = filter (\line -> not $ null line) $ lines content

	let packets1 = map parse $ rawPackets
	print $ comparePackets packets1 1

	let packets2 = map (fst . head) $ sort $ map parse $ "[[2]]":"[[6]]":rawPackets
	let f6 = fromJust $ findIndex (== Node [Node [Leaf 6]]) packets2
	let f2 = fromJust $ findIndex (== Node [Node [Leaf 2]]) packets2
	print $ (f2+1) * (f6+1)
