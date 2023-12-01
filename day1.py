import re

def read_file(file_path):
    def line_generator(file_path):
        with open(file_path, 'r', encoding='utf-8') as file:
            for line in file:
                yield line.strip()

    return line_generator(file_path)

input = read_file('data1.txt')
# tot = 0
# for line in input:
#     line = re.sub(r'\D', '', line)
#     tot+= int(line[0])*10+int(line[-1])

# print(tot)



def f(s):
  for c in s:
    if c.isdigit():
      return c


D = {"one":1,"1":1,"two":2,"2":2,"three":3,"3":3,"four":4,"4":4,"five":5,"5":5,
     "six":6,"6":6,"seven":7,"7":7,"eight":8,"8":8,"nine":9,"9":9}




dic = {"one":1, "two":2, "three":3, "four":4, "five":5, "six":6,"seven":7, "eight":8, "nine":9}
def look(input):
    first = 0
    last = 0
    inputfor = input
    inputback = input
    for i in range(len(input)):
        fstart = inputfor[:i]
        # fend = line[i+1:]
        fend = inputback[len(inputback)-i:]
        # if any(key in fstart for key in dic.keys()):
        #     fstart = fstart.replace(fstart, str(dic[]))
        for key in dic.keys():
            if key in fstart:
                fstart = fstart.replace(key, str(dic[key]))
                inputfor = fstart+inputfor[i:]
            if key in fend:
                fend = fend.replace(key, str(dic[key]))
                inputback = inputback[:len(inputback)-i]+fend
        # print(fstart, fend)
        if any(char.isdigit() for char in fstart):
            first = int(re.sub(r'\D', '', fstart)[0])
            # input[:i] = fstart
        if any(char.isdigit() for char in fend):
            last = int(re.sub(r'\D', '', fend)[-1])
            # input[len(input)-i:] = fend
        # print(first, last)
        if first!=0 and last!=0:
            return first*10+last
    # print(first, last)
    if first == 0 and last == 0:
        return 0
    if first == 0:
        return last*10+last
    if last == 0:
        return first*10+first
tot = 0





def g(s,rev=False):
  p,r = 999,0
  for n,v in D.items():
    if rev: n=n[::-1]
    print(n)
    k = s.find(n)
    if k<0: continue
    if k<p: p,r=k,v
  return r





for line in input:
    # res = look(line)
    # # print(res)
    # if res != g(line)*10+g(line[::-1],True):
    #     print(line)
    #     print(res)
    # if res < 10:
    #     print(line)
    # # print(res)
    # tot+=res
    print(g(line)*10+g(line[::-1],True))
# print(tot)