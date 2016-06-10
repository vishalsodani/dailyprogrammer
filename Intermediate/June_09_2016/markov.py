def markov():
	input = "Now is not the time for desert, now is the time for dinner"
	input = input.replace(',','').split()
	input_length = len(input)
	prefix_list = {}

	for i in range(0, input_length):
		if i < input_length - 2:
			if i > 0:
				suffix_list = []
				used = False
				for key, value in prefix_list.iteritems():
					if value[0] == input[i].lower() and value[1] == input[i+1].lower():
						for val in value[2]:
							if val != input[i+2].lower():
								suffix_list.append(val)
					else:
						if not used:
							suffix_list.append(input[i+2].lower())
							used = True

				prefix_list[i] = input[i].lower(), input[i+1].lower(), suffix_list
			else:
				prefix_list[0] = input[0].lower(), input[0+1].lower(), [input[0+2].lower()]

	for key, value in prefix_list.iteritems():
		print (value[0] + " " + value[1], ", ".join(value[2]))

if __name__ == "__main__":
    markov()
