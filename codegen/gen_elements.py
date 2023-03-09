import ast

BASE_CODE = '''
generate_element! {
    __element_name__;
    props: { __props__ }
}
'''

def build_prop_field(attr_name):
    attr_name = attr_name.replace('-', '_')
    return f'\n{indent(2)}{attr_name}: AttrValue,'

def indent(level = 1):
    return ' ' * (4 * level)

elements = ast.get_elements_with_attrs()

for (element_name, element_metadata) in elements.items():
    element_attrs = element_metadata['attrs']
    element_name = element_name.strip('<').strip('>')
    
    ending_char = '' if len(element_attrs) == 0 else '\n' + indent(1)
    
    code = BASE_CODE
    code = code.replace('__element_name__', element_name)
    code = code.replace('__props__', ''.join(map(build_prop_field, element_attrs)) + ending_char)
    code = code.strip()
    # print(code, '', sep='\n')

print(',\n'.join([f'''"{x.strip('<').strip('>')}"''' for x in elements.keys()]))