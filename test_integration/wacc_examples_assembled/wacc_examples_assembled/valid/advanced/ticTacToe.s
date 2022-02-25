	println "=  Because we know you want to win   =" ;
	
	char chosen = '\0' ;
	while chosen == '\0' do 
		print "Which symbol you would like to choose: " ;
		char c = '\0' ;
		read c ;
		if c == 'x' || c == 'X' then
			chosen = 'x'
		else
			if c == 'o' || c == 'O' then
				chosen = 'o'
			else
				if c == 'q' || c == 'Q' then
					println "Goodbye safety." ;
					exit 0
				else
					print "Invalid symbol: " ;
					println c ;
					println "Please try again."
				fi 
			fi
		fi
	done ;
	print "You have chosen: " ;
	println chosen ;
	return chosen
end

# Print the board out to the screen.
bool printBoard(pair(pair, pair) board) is
	pair(pair, pair) front = fst board ;
	pair(pair, char) row1 = fst front ;
	pair(pair, char) row2 = snd front ;
	pair(pair, char) row3 = snd board ;

   println " 1 2 3";
   print "1";	
	bool _ = call printRow(row1) ;
	println " -+-+-" ;
   print "2";	
	_ = call printRow(row2) ;
	println " -+-+-" ;
   print "3";	
	_ = call printRow(row3) ;
	println "";
	return true 
end

# Print a row with a newline to the screen.
bool printRow(pair(pair, char) row) is
	pair(char, char) front = fst row ;
	
	char cell1 = fst front ;
	char cell2 = snd front ;
	char cell3 = snd row ;
	
	bool _ = call printCell(cell1) ;
	print '|' ;
	_ = call printCell(cell2) ;
	print '|' ;
	_ = call printCell(cell3) ;
	println "" ;
	return true
end 

# Print a given cell. Print an empty space if it is empty. Return true.
bool printCell(char cell) is
	if cell == '\0' then
		print ' ' 
	else
		print cell 
	fi ;
	return true
end

# Ask for a move from the human player. The valid move is then stored in the given move array. 
# The row number is stored at move[0], the column number is stored at move[1]. Return true.
bool askForAMoveHuman(pair(pair, pair) board, int[] move) is
	bool success = false ;
	int row = 0 ;
	int column = 0 ;
		
	while !success do
		println "What is your next move?" ;
		print " row (1-3): " ;
		read row ;
		print " column (1-3): " ;
		read column ; 
		success = call validateMove(board, row, column) ;
		
		if success then
			println "" ; # Just print out an empty line
			move[0] = row ;
			move[1] = column ;
			return true
		else
			println "Your move is invalid. Please try again."
		fi			
	done ; 
	# Should not reach here
	return true
end

# Validate that the give move is valid. Returns true iff it is valid.
bool validateMove(pair(pair, pair) board, int moveRow, int moveColumn) is
	if 1 <= moveRow && moveRow <= 3 && 1 <= moveColumn && moveColumn <= 3 then
		char sym = call symbolAt(board, moveRow, moveColumn) ;
		# Make sure that the cell is empty
		return sym == '\0'
	else
		return false
	fi
end

# Print out to the screen about a recent move maid by the AI. Return true.
bool notifyMoveHuman(pair(pair, pair) board, char currentTurn, char playerSymbol, int moveRow, int moveColumn) is
	print "The AI played at row " ;
	print moveRow ;
	print " column " ;
	println moveColumn ;
	return true
end

############################### AI Functions #########################################

# Initialise an AI data.
pair(pair, pair) initAI(char aiSymbol) is
	
	pair(char, pair) info = newpair(aiSymbol, null) ; # Don't know yet how to use the second element.
		pair(pair, int) stateTree = call generateAllPossibleStates(aiSymbol) ;
	int value = call setValuesForAllStates(stateTree, aiSymbol, 'x') ;
	
	pair(pair, pair) aiData = newpair(info, stateTree) ;
	return aiData
end

# Generate the whole tree of all states. Then return the tree.
pair(pair, int) generateAllPossibleStates(char aiSymbol) is
	pair(pair, pair) board = call allocateNewBoard() ;
	pair(pair, int) rootState = call convertFromBoardToState(board) ;
	rootState = call generateNextStates(rootState, 'x') ;
	return rootState
end

# Convert from a board to a state.
# A state consists of 3 objects: the board, the pointers to the next states, and the value for this state (int).
# Therefore, we use the Pair4Three structure.
pair(pair, int) convertFromBoardToState(pair(pair, pair) board) is
	
	pair(pair, pair) pointers = call generateEmptyPointerBoard() ;
	pair(pair, pair) front = newpair(board, pointers) ;
	pair(pair, int) state = newpair(front, 0) ; # The initial value of 0 will be replaced.
	
	return state
end

# Allocate memory for the pointers to the next state.
# It looks like a board, but contains pointers (pairs) instead of chars.
pair(pair, pair) generateEmptyPointerBoard() is
	
	pair(pair, pair) row1 = call generateEmptyPointerRow() ;
	pair(pair, pair) row2 = call generateEmptyPointerRow() ;
	pair(pair, pair) row3 = call generateEmptyPointerRow() ;
	
	pair(pair, pair) front = newpair(row1, row2) ;
	pair(pair, pair) root = newpair(front, row3) ;
	return root
	
end

# Allocate memory for the 3 pointers to the next state of a row.
pair(pair, pair) generateEmptyPointerRow() is
	pair(pair, pair) front = newpair(null, null) ;
	pair(pair, pair) root = newpair(front, null) ;
	return root 
end

# Generate next states recursively. Returns the state.
pair(pair, int) generateNextStates(pair(pair, int) state, char currentPlayer) is
	pair(pair, pair) front = fst state ;
	
	pair(pair, pair) board = fst front ;
	pair(pair, pair) pointers = snd front ;
	
	char previousPlayer = call oppositeSymbol(currentPlayer) ;
	
	bool won = call hasWon(board, previousPlayer) ;
	
	if won then
		# The game ends. The winner is known.
		return state 
	else
		bool _ = call generateNextStatesBoard(board, pointers, currentPlayer) ;
		return state
	fi
	
end

# Generate Just the next states for every possible point on the board. Update the pointers accordingly. Return true.
bool generateNextStatesBoard(pair(pair, pair) board, pair(pair, pair) pointers, char currentPlayer) is
	pair(pair, pair) front = fst board ;
	
	pair(pair, char) row1 = fst front ;
	pair(pair, char) row2 = snd front ;
	pair(pair, char) row3 = snd board ;
	
	pair(pair, pair) frontP = fst pointers ;
	
	pair(pair, pair) row1P = fst frontP ;
	pair(pair, pair) row2P = snd frontP ;
	pair(pair, pair) row3P = snd pointers ;
	
	bool _ = call generateNextStatesRow(board, row1, row1P, currentPlayer, 1) ;
	_ = call generateNextStatesRow(board, row2, row2P, currentPlayer, 2) ;
	_ = call generateNextStatesRow(board, row3, row3P, currentPlayer, 3) ;
	
	return true
end

# Generate Just the next states for every possible point on the row. Update the pointers accordingly. Return true.
bool generateNextStatesRow(pair(pair, pair) board, pair(pair, char) row, pair(pair, pair) pointerRow, char currentPlayer, int rowNumber) is
	pair(char, char) front = fst row ;
	
	char cell1 = fst front ;
	char cell2 = snd front ;
	char cell3 = snd row ;
	
	pair(pair, pair) frontP = fst pointerRow ;
	
	fst frontP = call generateNextStatesCell(board, cell1, currentPlayer, rowNumber, 1) ;
	snd frontP = call generateNextStatesCell(board, cell2, currentPlayer, rowNumber, 2) ;
	snd pointerRow = call generateNextStatesCell(board, cell3, currentPlayer, rowNumber, 3) ;
	
	return true
end

# Generate Just the next states for the cell on the board. Returns the pointer to the next state.
pair(pair, int) generateNextStatesCell(pair(pair, pair) board, char cell, char currentPlayer, int rowNumber, int columnNumber) is
	if cell == '\0' then
		# If the cell is empty, generate the next state.
		pair(pair, pair) board2 = call cloneBoard(board) ;
		bool _ = call placeMove(board2, currentPlayer, rowNumber, columnNumber) ;
		pair(pair, int) state = call convertFromBoardToState(board2) ;
		char nextPlayer = call oppositeSymbol(currentPlayer) ;
		
		# Generate next states recursively and return it out.
		state = call generateNextStates(state, nextPlayer) ;
		return state
	else
		# If the cell is not empty, return null.
		return null
	fi
end

# Clone board.
pair(pair, pair) cloneBoard(pair(pair, pair) board) is
	pair(pair, pair) board2 = call allocateNewBoard() ; 
	bool _ = call copyBoard(board, board2) ;
	return board2 
end

# Copy the content of one board to another. Return true.
bool copyBoard(pair(pair, pair) from, pair(pair, pair) to) is
	pair(pair, pair) frontFrom = fst from ;
	pair(pair, char) row1From = fst frontFrom ;
	pair(pair, char) row2From = snd frontFrom ;
	pair(pair, char) row3From = snd from ;
	
	pair(pair, pair) frontTo = fst to ;
	pair(pair, char) row1To = fst frontTo ;
	pair(pair, char) row2To = snd frontTo ;
	pair(pair, char) row3To = snd to ;
			
	bool _ = call copyRow(row1From, row1To) ;		
	_ = call copyRow(row2From, row2To) ;
	_ = call copyRow(row3From, row3To) ;
			
	return true
end

# Copy from one board row to another. Return true.
bool copyRow(pair(pair, char) from, pair(pair, char) to) is
	pair(char, char) frontFrom = fst from ;
	pair(char, char) frontTo = fst to ;
	
	fst frontTo = fst frontFrom ;
	snd frontTo = snd frontFrom ;
	snd to = snd from ;
	return true
end

# Calculate the value of how good each state is using Minimax approach. 
# If AI wins, value = 100.
# If AI lose, value = -100.
# If Stalemate, value = 0.
# Otherwise, combine the values from the next states.
# If this state is null, then return -101 if it is a max state, 101 if it is a min state (thus those values will not be picked).
# Return the value.
int setValuesForAllStates(pair(pair, int) state, char aiSymbol, char currentPlayer) is
	int outValue = 0 ;
	if state == null then
		# The current state is impossible to reach.
		# Assign a value that will not be picked in the future.
		if currentPlayer == aiSymbol then
			# Later on, we will pick the lowest value (min). So we set this value high so that it will not be picked.
			outValue = 101
		else
			# Later on, we will pick the highest value (max). So we set this value low so that it will not be picked.
			outValue = -101
		fi
	else 
	
		pair(pair, pair) front = fst state ;
		
		pair(pair, pair) board = fst front ;
		pair(pair, pair) pointers = snd front ;
		
		char anotherPlayer = call oppositeSymbol(currentPlayer) ;
		
		# The current player is about to play. So if another player has won it already, the current player cannot play it.
		bool won = call hasWon(board, anotherPlayer) ;
	
		if won then
			if anotherPlayer == aiSymbol then
				outValue = 100 # We won
			else
				outValue = -100 # We lost
			fi 
		else
			bool hasEmptyCell = call containEmptyCell(board) ;
			if hasEmptyCell then
				# If can do next move, calculate the value from the next states.
				outValue = call calculateValuesFromNextStates(pointers, aiSymbol, anotherPlayer) ;
				
				# In order for the AI to choose the winning move immediately, we have to reduce the value for those not winning yet.
				# So if the next move has value 100, we set the value of this move 90.
				if outValue == 100 then
					outValue = 90 
				else
					skip
				fi
			else
				# Otherwise, it is a stalemate.
				outValue = 0 
			fi 
		fi ;
		snd state = outValue
	fi ;
	return outValue
end

# Calculate the values for each next state, then combine them to get the value of this state. Return the value.
int calculateValuesFromNextStates(pair(pair, pair) pointers, char aiSymbol, char playerOfNextState) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ;
	pair(pair, pair) row3 = snd pointers ;
	
	int value1 = call calculateValuesFromNextStatesRow(row1, aiSymbol, playerOfNextState) ;
	int value2 = call calculateValuesFromNextStatesRow(row2, aiSymbol, playerOfNextState) ;
	int value3 = call calculateValuesFromNextStatesRow(row3, aiSymbol, playerOfNextState) ;
	
	int out = call combineValue(aiSymbol, playerOfNextState, value1, value2, value3) ;
	return out
end

# Calculate the values for each next state in a row, then combine them to get the value of this row. Return the value.
int calculateValuesFromNextStatesRow(pair(pair, pair) rowPointers, char aiSymbol, char playerOfNextState) is
	pair(pair, pair) front = fst rowPointers ;
	
	pair(pair, int) state1 = fst front ;
	pair(pair, int) state2 = snd front ; 
	pair(pair, int) state3 = snd rowPointers ;
	
	int value1 = call setValuesForAllStates(state1, aiSymbol, playerOfNextState) ;
	int value2 = call setValuesForAllStates(state2, aiSymbol, playerOfNextState) ;
	int value3 = call setValuesForAllStates(state3, aiSymbol, playerOfNextState) ;
	
	int out = call combineValue(aiSymbol, playerOfNextState, value1, value2, value3) ;
	return out
end

int combineValue(char aiSymbol, char playerOfNextState, int value1, int value2, int value3) is
	int out = 0 ;
	if aiSymbol == playerOfNextState then
		# We move next so the human moves now. Pick the lowest value.
		out = call min3(value1, value2, value3)
	else
		# Human moves next so we move now. Pick the highest value.
		out = call max3(value1, value2, value3)
	fi ;
	return out
end

# Find the minimum of the three.
int min3(int a, int b, int c) is
	if a < b then
		if a < c then
			return a 
		else 
			return c
		fi
	else
		if b < c then
			return b
		else 
			return c
		fi
	fi
end

# Find the maximum of the three.
int max3(int a, int b, int c) is
	if a > b then
		if a > c then
			return a 
		else 
			return c
		fi
	else
		if b > c then
			return b
		else 
			return c
		fi
	fi
end

# Destroy all memory used by the AI. Return true.
bool destroyAI(pair(pair, pair) aiData) is
	
	pair(char, pair) info = fst aiData ;
		pair(pair, int) stateTree = snd aiData ;

	bool _ = call deleteStateTreeRecursively(stateTree) ;
	free info ;
	free aiData ;
	return true
end

# Ask the AI for a new move. Return true.
bool askForAMoveAI(pair(pair, pair) board, char currentTurn, char playerSymbol, pair(pair, pair) aiData, int[] move) is
	
	pair(char, pair) info = fst aiData ;
		pair(pair, int) stateTree = snd aiData ;
	
	pair(pair, pair) front = fst stateTree ;
	
	pair(pair, pair) pointers = snd front ;
	int stateValue = snd stateTree ;
	
	bool _ = call findTheBestMove(pointers, stateValue, move) ;		
	
	println "AI is cleaning up its memory..." ;
	# Update the state tree by using the new move.
	snd aiData = call deleteAllOtherChildren(pointers, move[0], move[1]) ;

	_ = call deleteThisStateOnly(stateTree) ;
	return true
end

# Given the pointers to all next states, pick the first one with the given stateValue and store the move in the the given array.
# Return true. 
bool findTheBestMove(pair(pair, pair) pointers, int stateValue, int[] move) is

	# We have a hack by changing the state value to 90 if the next state is 100. 
	# So if we have a state value of 90, look for the one with 100 first.
	# If found, use it. Otherwise, look for the one with 90.
	if stateValue == 90 then
		bool found = call findMoveWithGivenValue(pointers, 100, move) ;
		if found then
			return true
		else
			skip
		fi
	else
		skip
	fi ;
	
	# Normal case. Or when cannot find the child with 100.
	bool found = call findMoveWithGivenValue(pointers, stateValue, move) ;
	if found then
		return true
	else
		# Should not happen. Cannot find such move.
		println "Internal Error: cannot find the next move for the AI" ;
		exit -1
	fi
	
end

# Given the pointers to all next states, pick the first one with the given stateValue and store the move in the the given array.
# Return true in this case. Otherwise, the move array is untouched and return false. 
bool findMoveWithGivenValue(pair(pair, pair) pointers, int stateValue, int[] move) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ; 
	pair(pair, pair) row3 = snd pointers ;
	
	bool find = call findMoveWithGivenValueRow(row1, stateValue, move) ;
	if find then
		move[0] = 1
	else
		find = call findMoveWithGivenValueRow(row2, stateValue, move) ;
		if find then
			move[0] = 2
		else
			find = call findMoveWithGivenValueRow(row3, stateValue, move) ;
			if find then
				move[0] = 3
			else
				# Not found, return false.
				return false
			fi
		fi
	fi ;
	return true
end

# Given a row of pointers, pick the first one with the given stateValue and store in move[1], return true if such child state is found. Otherwise, return false and move[1] is untouched.
bool findMoveWithGivenValueRow(pair(pair, pair) rowPointers, int stateValue, int[] move) is
	
	pair(pair, pair) front = fst rowPointers ;
	
	pair(pair, int) cell1 = fst front ;
	pair(pair, int) cell2 = snd front ;
	pair(pair, int) cell3 = snd rowPointers ;
	
	bool find = call hasGivenStateValue(cell1, stateValue) ;
	if find then
		move[1] = 1
	else
		find = call hasGivenStateValue(cell2, stateValue) ;
		if find then
			move[1] = 2 
		else 
			find = call hasGivenStateValue(cell3, stateValue) ;
			if find then
				move[1] = 3
			else 
				return false
			fi
		fi
	fi ;
	return true
end

# Given a state, an a state value. Returns true iff the state has the given state value.
bool hasGivenStateValue(pair(pair, int) state, int stateValue) is
	if state == null then
		return false
	else
		int actual = snd state ;
		return actual == stateValue
	fi
end

# Notify a move made by a human player to the AI. Return true.
bool notifyMoveAI(pair(pair, pair) board, char currentTurn, char playerSymbol, pair(pair, pair) aiData, int moveRow, int moveColumn) is
	
	#pair(char, pair) info = fst aiData ; #unused
	pair(pair, int) stateTree = snd aiData ;
	
	pair(pair, pair) front = fst stateTree ;
	
	#pair(pair, pair) board = fst front ; #unused
	pair(pair, pair) pointers = snd front ;
	
	println "AI is cleaning up its memory..." ;
	
	# Set new state tree, remove all other children created by making other moves.
	snd aiData = call deleteAllOtherChildren(pointers, moveRow, moveColumn) ;
	bool _ = call deleteThisStateOnly(stateTree) ;
	return true
end

# Delete all decendent states apart from those made by moving a given move. Return the child state of that given move.
pair(pair, int) deleteAllOtherChildren(pair(pair, pair) pointers, int moveRow, int moveColumn) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ;
	pair(pair, pair) row3 = snd pointers ;

	# Find which row to keep or which rows to delete.
	pair(pair, pair) toKeepRow = null;
	pair(pair, pair) toDeleteRow1 = null;
	pair(pair, pair) toDeleteRow2 = null;
	
	if moveRow == 1 then
		toKeepRow = row1 ; 
		toDeleteRow1 = row2 ; 
		toDeleteRow2 = row3
	else 
		toDeleteRow1 = row1 ;
		if moveRow == 2 then
			toKeepRow = row2 ; 
			toDeleteRow2 = row3
		else
			# moveRow == 3
			toKeepRow = row3 ; 
			toDeleteRow2 = row2
		fi
	fi ;
	
	pair(pair, int) out = call deleteAllOtherChildrenRow(toKeepRow, moveColumn) ;
	bool _ = call deleteChildrenStateRecursivelyRow(toDeleteRow1) ;
	_ = call deleteChildrenStateRecursivelyRow(toDeleteRow2) ;
	
	return out
end

pair(pair, int) deleteAllOtherChildrenRow(pair(pair, pair) rowPointers, int moveColumn) is
	pair(pair, pair) front = fst rowPointers ;
	
	pair(pair, int) cell1 = fst front ;
	pair(pair, int) cell2 = snd front ;
	pair(pair, int) cell3 = snd rowPointers ;

	# Find which cell to keep or which cells to delete.
	pair(pair, int) toKeepCell = null;
	pair(pair, int) toDeleteCell1 = null;
	pair(pair, int) toDeleteCell2 = null;
	
	if moveColumn == 1 then
		toKeepCell = cell1 ; 
		toDeleteCell1 = cell2 ; 
		toDeleteCell2 = cell3
	else 
		toDeleteCell1 = cell1 ;
		if moveColumn == 2 then
			toKeepCell = cell2 ; 
			toDeleteCell2 = cell3
		else
			# moveColumn == 3
			toKeepCell = cell3 ; 
			toDeleteCell2 = cell2
		fi
	fi ;
	
	bool _ = call deleteStateTreeRecursively(toDeleteCell1) ;
	_ = call deleteStateTreeRecursively(toDeleteCell2) ;
	
	return toKeepCell
end

# Deallocate a given state and all its decendents.
bool deleteStateTreeRecursively(pair(pair, int) stateTree) is
	if stateTree == null then
		return true 
	else
		pair(pair, pair) front = fst stateTree ;
		
		pair(pair, pair) board = fst front ;
		pair(pair, pair) pointers = snd front ;
		
		bool _ = call deleteChildrenStateRecursively(pointers) ;
		_ = call deleteThisStateOnly(stateTree) ;
		return true
	fi		
end

# Given a state tree, deallocate the board, the pointers and the other pairs of this state only. The childrens are preserved. Return true.
bool deleteThisStateOnly(pair(pair, int) stateTree) is	
	pair(pair, pair) front = fst stateTree ;
	
	pair(pair, pair) board = fst front ;
	pair(pair, pair) pointers = snd front ;

	bool _ = call freeBoard(board) ;
	_ = call freePointers(pointers) ;
	free front ;
	free stateTree ;
	return true
end

bool freePointers(pair(pair, pair) pointers) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ;
	pair(pair, pair) row3 = snd pointers ;
	
	bool _ = call freePointersRow(row1) ;
	_ = call freePointersRow(row2) ;
	_ = call freePointersRow(row3) ;
	
	free front ;
	free pointers ;
	return true
end

bool freePointersRow(pair(pair, pair) rowPointers) is
	pair(pair, pair) front = fst rowPointers ;
	
	free front ;
	free rowPointers ;
	return true
end

# Deallocate all decendent states.
bool deleteChildrenStateRecursively(pair(pair, pair) pointers) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ;
	pair(pair, pair) row3 = snd pointers ;
	
	bool _ = call deleteChildrenStateRecursivelyRow(row1) ;
	_ = call deleteChildrenStateRecursivelyRow(row2) ;
	_ = call deleteChildrenStateRecursivelyRow(row3) ;
	
	return true
end

# Deallocate all decendent states given a row of pointers.
bool deleteChildrenStateRecursivelyRow(pair(pair, pair) rowPointers) is
	pair(pair, pair) front = fst rowPointers ;
	pair(pair, int) cell1 = fst front ;
	pair(pair, int) cell2 = snd front ;
	pair(pair, int) cell3 = snd rowPointers ;
	
	bool _ = call deleteStateTreeRecursively(cell1) ;
	_ = call deleteStateTreeRecursively(cell2) ;
	_ = call deleteStateTreeRecursively(cell3) ;
	
	return true
end

############################### Game Engine Functions ##################################

# Ask for a move from the current player. The valid move is stored in the move array. Return true.
bool askForAMove(pair(pair, pair) board, char currentTurn, char playerSymbol, pair(pair, pair) aiData, int[] move) is
	if currentTurn == playerSymbol then
		bool _ = call askForAMoveHuman(board, move)
	else 
		bool _ = call askForAMoveAI(board, currentTurn, playerSymbol, aiData, move)
	fi ;
	return true
end

# Place the given move of the currentTurn in the board. Return true.
bool placeMove(pair(pair, pair) board, char currentTurn, int moveRow, int moveColumn) is
	
	# Find the target row.
	pair(pair, char) targetRow = null ;
	if moveRow <= 2 then
		pair(pair, pair) front = fst board ;
		if moveRow == 1 then
			targetRow = fst front
		else
			# moveRow == 2
			targetRow = snd front
		fi
	else
		# moveRow == 3
		targetRow = snd board
	fi ;
	
	# Set the target cell
	if moveColumn <= 2 then
		pair(char, char) front = fst targetRow ;
		if moveColumn == 1 then
			fst front = currentTurn
		else
			# moveColumn == 2
			snd front = currentTurn
		fi
	else
		# moveColumn == 3
		snd targetRow = currentTurn
	fi ;
	return true
	
end

# Notify the opponent about a move of another party. Return true.
bool notifyMove(pair(pair, pair) board, char currentTurn, char playerSymbol, pair(pair, pair) aiData, int moveRow, int moveColumn) is
	if currentTurn == playerSymbol then
		bool _ = call notifyMoveAI(board, currentTurn, playerSymbol, aiData, moveRow, moveColumn)
	else 
		bool _ = call notifyMoveHuman(board, currentTurn, playerSymbol, moveRow, moveColumn)
	fi ;
	return true
end

# Given either 'x' or 'o', returns another one.
char oppositeSymbol(char symbol) is
	if symbol == 'x' then
		return 'o' 
	else
		if symbol == 'o' then
			return 'x'
		else
			# Should not happen!
			println "Internal Error: symbol given is neither \'x\' or \'o\'" ;
			exit -1 
		fi 
	fi
end

# row = 1, 2 or 3
# column = 1, 2 or 3
char symbolAt(pair(pair, pair) board, int row, int column) is

	# Find the target row.
	pair(pair, char) targetRow = null ;
	if row <= 2 then
		pair(pair, pair) front = fst board ;
		if row == 1 then
			targetRow = fst front
		else
			# row == 2
			targetRow = snd front
		fi
	else
		# row == 3
		targetRow = snd board
	fi ;
	
	# Now find the target cell.
	char targetCell = '\0' ;
	if column <= 2 then
		pair(char, char) front = fst targetRow ;
		if column == 1 then 
			targetCell = fst front 
		else
			# column == 2
			targetCell = snd front
		fi
	else
		# column == 3
		targetCell = snd targetRow
	fi ;
		
	return targetCell	
end

# Return true if there is at least one empty cell where the next player can place a move. Otherwise, return false (game ends).
bool containEmptyCell(pair(pair, pair) board) is
	pair(pair, pair) front = fst board ;
	
	pair(pair, char) row1 = fst front ;
	pair(pair, char) row2 = snd front ;
	pair(pair, char) row3 = snd board ;
	
	bool row1ContainEmpty = call containEmptyCellRow(row1) ;
	bool row2ContainEmpty = call containEmptyCellRow(row2) ;
	bool row3ContainEmpty = call containEmptyCellRow(row3) ;
	
	return row1ContainEmpty || row2ContainEmpty || row3ContainEmpty
end

bool containEmptyCellRow(pair(pair, char) row) is
	pair(char, char) front = fst row ;
	
	char cell1 = fst front ;
	char cell2 = snd front ;
	char cell3 = snd row ;
	
	return cell1 == '\0' || cell2 == '\0' || cell3 == '\0'
end

# Find if the candidate symbol ('x' or 'o') has won the game.
# Returns true if and only if it has won. 
bool hasWon(pair(pair, pair) board, char candidate) is
	char c11 = call symbolAt(board, 1, 1) ;
	char c12 = call symbolAt(board, 1, 2) ;
	char c13 = call symbolAt(board, 1, 3) ;
	char c21 = call symbolAt(board, 2, 1) ;
	char c22 = call symbolAt(board, 2, 2) ;
	char c23 = call symbolAt(board, 2, 3) ;
	char c31 = call symbolAt(board, 3, 1) ;
	char c32 = call symbolAt(board, 3, 2) ;
	char c33 = call symbolAt(board, 3, 3) ;
	
	return 
		# Row win
		c11 == candidate && c12 == candidate && c13 == candidate ||
		c21 == candidate && c22 == candidate && c23 == candidate ||
		c31 == candidate && c32 == candidate && c33 == candidate ||
		 
		# Column win
		c11 == candidate && c21 == candidate && c31 == candidate ||
		c12 == candidate && c22 == candidate && c32 == candidate ||
		c13 == candidate && c23 == candidate && c33 == candidate ||
		
		# Diagonal win
		c11 == candidate && c22 == candidate && c33 == candidate ||
		c13 == candidate && c22 == candidate && c31 == candidate
end

# Allocate a new board. 
# We use a Pair4Three structure to store pointers to the 3 rows.
pair(pair, pair) allocateNewBoard() is
	pair(pair, char) row1 = call allocateNewRow() ;
	pair(pair, char) row2 = call allocateNewRow() ;
	pair(pair, char) row3 = call allocateNewRow() ;
	
	pair(pair, pair) front = newpair(row1, row2) ;
	pair(pair, pair) root = newpair(front, row3) ;
	return root
end

# Allocate a row of the board. 
# A row is represented by a Pair4Three structure.
# The default value in each cell is '\0'.
pair(pair, char) allocateNewRow() is
	pair(char, char) front = newpair('\0', '\0') ;
	pair(pair, char) root = newpair(front, '\0') ;
	return root
end

# Free a memory used to store the whole board.
# Return true.
bool freeBoard(pair(pair, pair) board) is
	pair(pair, pair) front = fst board ;
	
	pair(pair, char) row1 = fst front ;
	pair(pair, char) row2 = snd front ;
	pair(pair, char) row3 = snd board ;
	
	bool _ = call freeRow(row1) ;
	_ = call freeRow(row2) ;
	_ = call freeRow(row3) ;
	
	free front ;
	free board ;
	return true
end

# Free the memory used for a row. Return true.
bool freeRow(pair(pair, char) row) is
	pair(char, char) front = fst row ;
	free front ;
	free row ;
	return true
end

# For debugging purpose.
bool printAiData(pair(pair, pair) aiData) is
	
	pair(char, pair) info = fst aiData ;
	pair(pair, int) stateTree = snd aiData ;
	
	bool _ = call printStateTreeRecursively(stateTree) ;
	exit 0
end

bool printStateTreeRecursively(pair(pair, int) stateTree) is
	if stateTree == null then
		return true 
	else 
		pair(pair, pair) front = fst stateTree ;
		
		pair(pair, pair) board = fst front ;
		pair(pair, pair) pointers = snd front ;
		int value = snd stateTree ;
		
		# Print the value
		print 'v' ;
		print '=' ;
		println value ;
		
		bool _ = call printBoard(board) ;
		_ = call printChildrenStateTree(pointers) ;
		
		println 'p' ;
		return true
	fi
end

bool printChildrenStateTree(pair(pair, pair) pointers) is
	pair(pair, pair) front = fst pointers ;
	
	pair(pair, pair) row1 = fst front ;
	pair(pair, pair) row2 = snd front ;
	pair(pair, pair) row3 = snd pointers ;
	
	bool _ = call printChildrenStateTreeRow(row1) ;
	_ = call printChildrenStateTreeRow(row2) ;
	_ = call printChildrenStateTreeRow(row3) ;
	return true
end

bool printChildrenStateTreeRow(pair(pair, pair) rowPointers) is
	pair(pair, pair) front = fst rowPointers ;
	
	pair(pair, int) cell1 = fst front ;
	pair(pair, int) cell2 = snd front ;
	pair(pair, int) cell3 = snd rowPointers ;
	
	bool _ = call printStateTreeRecursively(cell1) ;
	_ = call printStateTreeRecursively(cell2) ;
	_ = call printStateTreeRecursively(cell3) ;
	
	return true
end

############################## Main Function ############################

char playerSymbol = call chooseSymbol() ;
char aiSymbol = call oppositeSymbol(playerSymbol) ;
char currentTurn = 'x' ;

pair(pair, pair) board = call allocateNewBoard() ;

println "Initialising AI. Please wait, this may take a few minutes." ;
pair(pair, pair) aiData = call initAI(aiSymbol) ;

int turnCount = 0 ;
char winner = '\0' ;

bool _ = call printBoard(board) ;

while winner == '\0' && turnCount < 9 do
	int[] move = [0, 0] ;
	_ = call askForAMove(board, currentTurn, playerSymbol, aiData, move) ;
	_ = call placeMove(board, currentTurn, move[0], move[1]) ;
	_ = call notifyMove(board, currentTurn, playerSymbol, aiData, move[0], move[1]) ;
	_ = call printBoard(board) ;
	bool won = call hasWon(board, currentTurn) ;
	if won then
		winner = currentTurn
	else 
		skip
	fi ;
	
	# Progress to the next turn
	currentTurn = call oppositeSymbol(currentTurn) ;
	turnCount = turnCount + 1
done ;

_ = call freeBoard(board) ;
_ = call destroyAI(aiData) ;

if winner != '\0' then
	print winner ;
	println " has won!" 
else 
	println "Stalemate!" 
fi
nd
--------------------------------------------------------------

- Compiler Output:
- Compiling...
- Printing Assembly...
icTacToe.s contents are:
msg_1:
	.word 38
	.ascii	"=  Because we know you want to win   ="
msg_2:
	.word 38
msg_10:
	.word 39
	.ascii	"Which symbol you would like to choose: "
msg_11:
	.word 15
	.ascii	"Goodbye safety."
msg_12:
	.word 16
	.ascii	"Invalid symbol: "
msg_13:
	.word 17
	.ascii	"Please try again."
msg_14:
	.word 17
	.ascii	"You have chosen: "
msg_15:
	.word 6
	.ascii	" 1 2 3"
msg_16:
	.word 1
	.ascii	"1"
msg_17:
	.word 6
	.ascii	" -+-+-"
msg_18:
	.word 1
	.ascii	"2"
msg_19:
	.word 6
	.ascii	" -+-+-"
msg_20:
	.word 1
	.ascii	"3"
msg_21:
	.word 0
	.ascii	""
msg_22:
	.word 0
	.ascii	""
msg_23:
	.word 23
	.ascii	"What is your next move?"
msg_24:
	.word 12
	.ascii	" row (1-3): "
msg_25:
	.word 15
	.ascii	" column (1-3): "
msg_26:
	.word 0
	.ascii	""
msg_27:
	.word 39
	.ascii	"Your move is invalid. Please try again."
msg_28:
	.word 21
	.ascii	"The AI played at row "
msg_29:
	.word 8
	.ascii	" column "
msg_30:
	.word 31
	.ascii	"AI is cleaning up its memory..."
msg_31:
	.word 52
	.ascii	"Internal Error: cannot find the next move for the AI"
msg_32:
	.word 31
	.ascii	"AI is cleaning up its memory..."
msg_33:
	.word 50
	.ascii	"Internal Error: symbol given is neither \'x\' or \'o\'"
msg_34:
	.word 58
	.ascii	"Initialising AI. Please wait, this may take a few minutes."
msg_35:
	.word 9
	.ascii	" has won!"
msg_36:
	.word 10
	.ascii	"Stalemate!"
msg_37:
	.word 5
	.ascii	"%.*s\0"
msg_38:
	.word 1
	.ascii	"\0"
msg_39:
	.word 4
	.ascii	" %c\0"
msg_40:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_41:
	.word 3
	.ascii	"%d\0"
msg_42:
	.word 44
	.ascii	"ArrayIndexOutOfBoundsError: negative index\n\0"
msg_43:
	.word 45
	.ascii	"ArrayIndexOutOfBoundsError: index too large\n\0"
msg_44:
	.word 3
	.ascii	"%d\0"
msg_45:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_46:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"

.text

.global main
f_chooseSymbol:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_6
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_7
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_8
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_9
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #0
	STRB r4, [sp]
	B L0
L1:
	SUB sp, sp, #1
	LDR r4, =msg_10
	MOV r0, r4
	BL p_print_string
	MOV r4, #0
	STRB r4, [sp]
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_char
	LDRSB r4, [sp]
	MOV r5, #'x'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'X'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L2
	MOV r4, #'x'
	STRB r4, [sp, #1]
	B L3
L2:
	LDRSB r4, [sp]
	MOV r5, #'o'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'O'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L4
	MOV r4, #'o'
	STRB r4, [sp, #1]
	B L5
L4:
	LDRSB r4, [sp]
	MOV r5, #'q'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'Q'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L6
	LDR r4, =msg_11
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =0
	MOV r0, r4
	BL exit
	B L7
L6:
	LDR r4, =msg_12
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDR r4, =msg_13
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L7:
L5:
L3:
	ADD sp, sp, #1
L0:
	LDRSB r4, [sp]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #1
	BEQ L1
	LDR r4, =msg_14
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDRSB r4, [sp]
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	POP {pc}
	.ltorg
f_printBoard:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =msg_15
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_16
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_17
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_18
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_19
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_20
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_21
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_printRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #3]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #2]
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #3]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'|'
	MOV r0, r4
	BL putchar
	LDRSB r4, [sp, #2]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'|'
	MOV r0, r4
	BL putchar
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_22
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_printCell:
	PUSH {lr}
	LDRSB r4, [sp, #4]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L8
	MOV r4, #' '
	MOV r0, r4
	BL putchar
	B L9
L8:
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL putchar
L9:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMoveHuman:
	PUSH {lr}
	SUB sp, sp, #9
	MOV r4, #0
	STRB r4, [sp, #8]
	LDR r4, =0
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L10
L11:
	LDR r4, =msg_23
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_24
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_25
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_validateMove
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #8]
	LDRSB r4, [sp, #8]
	CMP r4, #0
	BEQ L12
	LDR r4, =msg_26
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #4]
	ADD r5, sp, #17
	LDR r6, =0
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	LDR r4, [sp]
	ADD r6, sp, #17
	LDR r7, =1
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	B L13
L12:
	LDR r4, =msg_27
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L13:
L10:
	LDRSB r4, [sp, #8]
	EOR r4, r4, #1
	CMP r4, #1
	BEQ L11
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_validateMove:
	PUSH {lr}
	LDR r4, =1
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	LDR r5, [sp, #8]
	LDR r6, =3
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	LDR r5, =1
	LDR r6, [sp, #12]
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	LDR r5, [sp, #12]
	LDR r6, =3
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	CMP r4, #0
	BEQ L14
	SUB sp, sp, #1
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	ADD sp, sp, #1
	B L15
L14:
	MOV r4, #0
	MOV r0, r4
	POP {pc}
L15:
	POP {pc}
	.ltorg
f_notifyMoveHuman:
	PUSH {lr}
	LDR r4, =msg_28
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_29
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #14]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_initAI:
	PUSH {lr}
	SUB sp, sp, #16
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDRSB r5, [sp, #20]
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #12]
	LDRSB r4, [sp, #20]
	STRB r4, [sp, #-1]!
	BL f_generateAllPossibleStates
	ADD sp, sp, #1
	MOV r4, r0
	STR r4, [sp, #8]
	MOV r4, #'x'
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #21]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #16
	POP {pc}
	POP {pc}
	.ltorg
f_generateAllPossibleStates:
	PUSH {lr}
	SUB sp, sp, #8
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_convertFromBoardToState
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	MOV r4, #'x'
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_generateNextStates
	ADD sp, sp, #5
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_convertFromBoardToState:
	PUSH {lr}
	SUB sp, sp, #12
	BL f_generateEmptyPointerBoard
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #12
	POP {pc}
	POP {pc}
	.ltorg
f_generateEmptyPointerBoard:
	PUSH {lr}
	SUB sp, sp, #20
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #16]
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #12]
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #20
	POP {pc}
	POP {pc}
	.ltorg
f_generateEmptyPointerRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStates:
	PUSH {lr}
	SUB sp, sp, #14
	LDR r4, [sp, #18]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #10]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #6]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #2]
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L16
	LDR r4, [sp, #18]
	MOV r0, r4
	ADD sp, sp, #14
	POP {pc}
	B L17
L16:
	SUB sp, sp, #1
	LDRSB r4, [sp, #23]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesBoard
	ADD sp, sp, #9
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #19]
	MOV r0, r4
	ADD sp, sp, #15
	POP {pc}
	ADD sp, sp, #1
L17:
	POP {pc}
	.ltorg
f_generateNextStatesBoard:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #34]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #30]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStatesRow:
	PUSH {lr}
	SUB sp, sp, #11
	LDR r4, [sp, #19]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #7]
	LDR r4, [sp, #7]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #6]
	LDR r4, [sp, #7]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #5]
	LDR r4, [sp, #19]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #15]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STR r4, [r5]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #14]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #13]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp, #23]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #11
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStatesCell:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L18
	SUB sp, sp, #10
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	BL f_cloneBoard
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #6]
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_placeMove
	ADD sp, sp, #13
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	BL f_convertFromBoardToState
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #1]
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #2]
	STR r4, [sp, #-4]!
	BL f_generateNextStates
	ADD sp, sp, #5
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #10
	POP {pc}
	ADD sp, sp, #10
	B L19
L18:
	LDR r4, =0
	MOV r0, r4
	POP {pc}
L19:
	POP {pc}
	.ltorg
f_cloneBoard:
	PUSH {lr}
	SUB sp, sp, #5
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_copyBoard
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_copyBoard:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_copyRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STRB r4, [r5]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	LDR r5, [sp, #16]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_setValuesForAllStates:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L20
	LDRSB r4, [sp, #13]
	LDRSB r5, [sp, #12]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L22
	LDR r4, =101
	STR r4, [sp]
	B L23
L22:
	LDR r4, =-101
	STR r4, [sp]
L23:
	B L21
L20:
	SUB sp, sp, #14
	LDR r4, [sp, #22]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #10]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #6]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #2]
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L24
	LDRSB r4, [sp, #1]
	LDRSB r5, [sp, #26]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L26
	LDR r4, =100
	STR r4, [sp, #14]
	B L27
L26:
	LDR r4, =-100
	STR r4, [sp, #14]
L27:
	B L25
L24:
	SUB sp, sp, #1
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_containEmptyCell
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L28
	LDRSB r4, [sp, #2]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #28]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #15]
	LDR r4, [sp, #15]
	LDR r5, =100
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L30
	LDR r4, =90
	STR r4, [sp, #15]
	B L31
L30:
L31:
	B L29
L28:
	LDR r4, =0
	STR r4, [sp, #15]
L29:
	ADD sp, sp, #1
L25:
	LDR r4, [sp, #14]
	LDR r5, [sp, #22]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	ADD sp, sp, #14
L21:
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_calculateValuesFromNextStates:
	PUSH {lr}
	SUB sp, sp, #32
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #28]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #24]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #20]
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #16]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #12]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #22]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #8]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	BL f_combineValue
	ADD sp, sp, #14
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #32
	POP {pc}
	POP {pc}
	.ltorg
f_calculateValuesFromNextStatesRow:
	PUSH {lr}
	SUB sp, sp, #32
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #28]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #24]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #20]
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #16]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #12]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #22]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #8]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	BL f_combineValue
	ADD sp, sp, #14
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #32
	POP {pc}
	POP {pc}
	.ltorg
f_combineValue:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L32
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_min3
	ADD sp, sp, #12
	MOV r4, r0
	STR r4, [sp]
	B L33
L32:
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_max3
	ADD sp, sp, #12
	MOV r4, r0
	STR r4, [sp]
L33:
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_min3:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L34
	LDR r4, [sp, #4]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L36
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	B L37
L36:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L37:
	B L35
L34:
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L38
	LDR r4, [sp, #8]
	MOV r0, r4
	POP {pc}
	B L39
L38:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L39:
L35:
	POP {pc}
	.ltorg
f_max3:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L40
	LDR r4, [sp, #4]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L42
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	B L43
L42:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L43:
	B L41
L40:
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L44
	LDR r4, [sp, #8]
	MOV r0, r4
	POP {pc}
	B L45
L44:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L45:
L41:
	POP {pc}
	.ltorg
f_destroyAI:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMoveAI:
	PUSH {lr}
	SUB sp, sp, #21
	LDR r4, [sp, #31]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #31]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #35]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findTheBestMove
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_30
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD r4, sp, #35
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #39
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildren
	ADD sp, sp, #12
	MOV r4, r0
	LDR r5, [sp, #31]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #21
	POP {pc}
	POP {pc}
	.ltorg
f_findTheBestMove:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, [sp, #9]
	LDR r5, =90
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L46
	SUB sp, sp, #1
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	LDR r4, =100
	STR r4, [sp, #-4]!
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValue
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L48
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #2
	POP {pc}
	B L49
L48:
L49:
	ADD sp, sp, #1
	B L47
L46:
L47:
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValue
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L50
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	B L51
L50:
	LDR r4, =msg_31
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =-1
	MOV r0, r4
	BL exit
L51:
	POP {pc}
	.ltorg
f_findMoveWithGivenValue:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L52
	LDR r4, =1
	ADD r5, sp, #29
	LDR r6, =0
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	B L53
L52:
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L54
	LDR r4, =2
	ADD r6, sp, #29
	LDR r7, =0
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	B L55
L54:
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L56
	LDR r4, =3
	ADD r7, sp, #29
	LDR r8, =0
	LDR r7, [r7]
	MOV r0, r8
	MOV r1, r7
	BL p_check_array_bounds
	ADD r7, r7, #4
	ADD r7, r7, r8, LSL #2
	STR r4, [r7]
	B L57
L56:
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
L57:
L55:
L53:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_findMoveWithGivenValueRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L58
	LDR r4, =1
	ADD r5, sp, #29
	LDR r6, =1
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	B L59
L58:
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L60
	LDR r4, =2
	ADD r6, sp, #29
	LDR r7, =1
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	B L61
L60:
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L62
	LDR r4, =3
	ADD r7, sp, #29
	LDR r8, =1
	LDR r7, [r7]
	MOV r0, r8
	MOV r1, r7
	BL p_check_array_bounds
	ADD r7, r7, #4
	ADD r7, r7, r8, LSL #2
	STR r4, [r7]
	B L63
L62:
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
L63:
L61:
L59:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_hasGivenStateValue:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L64
	MOV r4, #0
	MOV r0, r4
	POP {pc}
	B L65
L64:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	ADD sp, sp, #4
L65:
	POP {pc}
	.ltorg
f_notifyMoveAI:
	PUSH {lr}
	SUB sp, sp, #13
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =msg_32
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #31]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #31]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildren
	ADD sp, sp, #12
	MOV r4, r0
	LDR r5, [sp, #23]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	POP {pc}
	.ltorg
f_deleteAllOtherChildren:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, =0
	STR r4, [sp, #13]
	LDR r4, =0
	STR r4, [sp, #9]
	LDR r4, =0
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L66
	LDR r4, [sp, #25]
	STR r4, [sp, #13]
	LDR r4, [sp, #21]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	B L67
L66:
	LDR r4, [sp, #25]
	STR r4, [sp, #9]
	LDR r4, [sp, #41]
	LDR r5, =2
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L68
	LDR r4, [sp, #21]
	STR r4, [sp, #13]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	B L69
L68:
	LDR r4, [sp, #17]
	STR r4, [sp, #13]
	LDR r4, [sp, #21]
	STR r4, [sp, #5]
L69:
L67:
	LDR r4, [sp, #45]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildrenRow
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_deleteAllOtherChildrenRow:
	PUSH {lr}
	SUB sp, sp, #29
	LDR r4, [sp, #33]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #25]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #25]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #33]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, =0
	STR r4, [sp, #9]
	LDR r4, =0
	STR r4, [sp, #5]
	LDR r4, =0
	STR r4, [sp, #1]
	LDR r4, [sp, #37]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L70
	LDR r4, [sp, #21]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	STR r4, [sp, #1]
	B L71
L70:
	LDR r4, [sp, #21]
	STR r4, [sp, #5]
	LDR r4, [sp, #37]
	LDR r5, =2
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L72
	LDR r4, [sp, #17]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	STR r4, [sp, #1]
	B L73
L72:
	LDR r4, [sp, #13]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #1]
L73:
L71:
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #9]
	MOV r0, r4
	ADD sp, sp, #29
	POP {pc}
	POP {pc}
	.ltorg
f_deleteStateTreeRecursively:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L74
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	B L75
L74:
	SUB sp, sp, #13
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	ADD sp, sp, #13
L75:
	POP {pc}
	.ltorg
f_deleteThisStateOnly:
	PUSH {lr}
	SUB sp, sp, #13
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freeBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freePointers
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	POP {pc}
	.ltorg
f_freePointers:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_freePointersRow:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_deleteChildrenStateRecursively:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_deleteChildrenStateRecursivelyRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMove:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L76
	SUB sp, sp, #1
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_askForAMoveHuman
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L77
L76:
	SUB sp, sp, #1
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_askForAMoveAI
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
L77:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_placeMove:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #13]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L78
	SUB sp, sp, #4
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L80
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	B L81
L80:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #4]
L81:
	ADD sp, sp, #4
	B L79
L78:
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
L79:
	LDR r4, [sp, #17]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L82
	SUB sp, sp, #4
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #21]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L84
	LDRSB r4, [sp, #16]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STRB r4, [r5]
	B L85
L84:
	LDRSB r4, [sp, #16]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
L85:
	ADD sp, sp, #4
	B L83
L82:
	LDRSB r4, [sp, #12]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
L83:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_notifyMove:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L86
	SUB sp, sp, #1
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	BL f_notifyMoveAI
	ADD sp, sp, #18
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L87
L86:
	SUB sp, sp, #1
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_notifyMoveHuman
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
L87:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_oppositeSymbol:
	PUSH {lr}
	LDRSB r4, [sp, #4]
	MOV r5, #'x'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L88
	MOV r4, #'o'
	MOV r0, r4
	POP {pc}
	B L89
L88:
	LDRSB r4, [sp, #4]
	MOV r5, #'o'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L90
	MOV r4, #'x'
	MOV r0, r4
	POP {pc}
	B L91
L90:
	LDR r4, =msg_33
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =-1
	MOV r0, r4
	BL exit
L91:
L89:
	POP {pc}
	.ltorg
f_symbolAt:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =0
	STR r4, [sp, #1]
	LDR r4, [sp, #13]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L92
	SUB sp, sp, #4
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L94
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	B L95
L94:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
L95:
	ADD sp, sp, #4
	B L93
L92:
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
L93:
	MOV r4, #0
	STRB r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L96
	SUB sp, sp, #4
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #21]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L98
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
	B L99
L98:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
L99:
	ADD sp, sp, #4
	B L97
L96:
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
L97:
	LDRSB r4, [sp]
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_containEmptyCell:
	PUSH {lr}
	SUB sp, sp, #19
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #15]
	LDR r4, [sp, #15]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #11]
	LDR r4, [sp, #15]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #7]
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #3]
	LDR r4, [sp, #11]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #2]
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #1]
	LDR r4, [sp, #3]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #2]
	LDRSB r5, [sp, #1]
	ORR r4, r4, r5
	LDRSB r5, [sp]
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #19
	POP {pc}
	POP {pc}
	.ltorg
f_containEmptyCellRow:
	PUSH {lr}
	SUB sp, sp, #7
	LDR r4, [sp, #11]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #3]
	LDR r4, [sp, #3]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #2]
	LDR r4, [sp, #3]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #1]
	LDR r4, [sp, #11]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
	LDRSB r4, [sp, #2]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp, #1]
	MOV r6, #0
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	LDRSB r5, [sp]
	MOV r6, #0
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #7
	POP {pc}
	POP {pc}
	.ltorg
f_hasWon:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #8]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #7]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #6]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #4]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #3]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #2]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #1]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #17]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp, #7]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	AND r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	AND r4, r4, r5
	LDRSB r5, [sp, #5]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #3]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #2]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #1]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #8]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #5]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #2]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #7]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #1]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #3]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #8]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #2]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_allocateNewBoard:
	PUSH {lr}
	SUB sp, sp, #20
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #16]
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #12]
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #20
	POP {pc}
	POP {pc}
	.ltorg
f_allocateNewRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r0, =8
	BL malloc
	MOV r4, r0
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_freeBoard:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_freeRow:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_printAiData:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =0
	MOV r0, r4
	BL exit
	POP {pc}
	.ltorg
f_printStateTreeRecursively:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L100
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	B L101
L100:
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	MOV r4, #'v'
	MOV r0, r4
	BL putchar
	MOV r4, #'='
	MOV r0, r4
	BL putchar
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTree
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'p'
	MOV r0, r4
	BL putchar
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	ADD sp, sp, #17
L101:
	POP {pc}
	.ltorg
f_printChildrenStateTree:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_printChildrenStateTreeRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #17
	BL f_chooseSymbol
	MOV r4, r0
	STRB r4, [sp, #16]
	LDRSB r4, [sp, #16]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #15]
	MOV r4, #'x'
	STRB r4, [sp, #14]
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #10]
	LDR r4, =msg_34
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDRSB r4, [sp, #15]
	STRB r4, [sp, #-1]!
	BL f_initAI
	ADD sp, sp, #1
	MOV r4, r0
	STR r4, [sp, #6]
	LDR r4, =0
	STR r4, [sp, #2]
	MOV r4, #0
	STRB r4, [sp, #1]
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	B L102
L103:
	SUB sp, sp, #5
	LDR r0, =12
	BL malloc
	MOV r4, r0
	LDR r5, =0
	STR r5, [r4, #4]
	LDR r5, =0
	STR r5, [r4, #8]
	LDR r5, =2
	STR r5, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #29]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #28]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_askForAMove
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp, #5]
	ADD r4, sp, #1
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #5
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	BL f_placeMove
	ADD sp, sp, #13
	MOV r4, r0
	STRB r4, [sp, #5]
	ADD r4, sp, #1
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #5
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #33]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #32]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	BL f_notifyMove
	ADD sp, sp, #18
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #5]
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L104
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #6]
	B L105
L104:
L105:
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #19]
	LDR r4, [sp, #7]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #7]
	ADD sp, sp, #5
L102:
	LDRSB r4, [sp, #1]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDR r5, [sp, #2]
	LDR r6, =9
	CMP r5, r6
	MOVLT r5, #1
	MOVGE r5, #0
	AND r4, r4, r5
	CMP r4, #1
	BEQ L103
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_freeBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	BL f_destroyAI
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #1]
	MOV r5, #0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #0
	BEQ L106
	LDRSB r4, [sp, #1]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_35
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L107
L106:
	LDR r4, =msg_36
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L107:
	ADD sp, sp, #17
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_37
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_38
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_39
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_40
	BLEQ p_throw_runtime_error
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_41
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_check_array_bounds:
	PUSH {lr}
	CMP r0, #0
	LDRLT r0, =msg_42
	BLLT p_throw_runtime_error
	LDR r1, [r1]
	CMP r0, r1
	LDRCS r0, =msg_43
	BLCS p_throw_runtime_error
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_44
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_free_pair:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_45
	BEQ p_throw_runtime_error
	PUSH {r0}
	LDR r0, [r0]
	BL free
	LDR r0, [sp]
	LDR r0, [r0, #4]
	BL free
	POP {r0}
	BL free
	POP {pc}
p_throw_overflow_error:
	LDR r0, =msg_46
	BL p_throw_runtime_error
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
