#!/usr/bin/env python3
import os
self = os.path.abspath(__file__)
root = os.path.dirname(self)
target = os.path.join(root, 'vmcode.go')
outfile = os.path.join(root, 'vmcodename.go')
with open(target, 'rt', encoding='utf-8') as fp:
    txt = fp.read()
# get vmcode
_,txt = txt.split('__begin__')
txt,_ = txt.split('__end__')
res = ''
for line in txt.split("\n"):
    line = line.strip()
    if line == '': continue
    if line[0:2] == '//': continue
    line += "//"
    code,comment = line.split('//')[0:2]
    code = code.split(' ')[0]
    code = code.strip()
    comment = comment.strip()
    res += '\t{0}:"{0}", // {1}\n'.format(code, comment)

# CODE
result = '''// auto generated by gen_vmcodename.py
package nadesiko4

var vmcodename = map[VMCodeType]string{
__CODE__
}

func getVMCodeName(id VMCodeType) string {
    n,ok := vmcodename[id]
    if ok {
        return n
    }
    return "unknown"
}

'''
result = result.replace('__CODE__', res)
# print(result)
with open(outfile, 'wt', encoding='utf-8') as fp:
    fp.write(result)
print("ok")


