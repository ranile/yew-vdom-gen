from bs4 import BeautifulSoup

elements_with_default = [
    "<style>",
    "<title>",
    "<h1>",
    "<h2>",
    "<h3>",
    "<h4>",
    "<h5>",
    "<h6>",
    "<p>",
    "<a>",
    "<abbr>",
    "<b>",
    "<bdi>",
    "<code>",
    "<dfn>",
    "<em>",
    "<i>",
    "<small>",
    "<strong>",
    "<script>",
    "<button>",
    "<label>"
]


def get_elements():
    with open('elements') as f:
        contents = f.read()

    soup = BeautifulSoup(contents, features="html.parser")

    find = soup.find(class_='main-page-content')
    children = list(find.children)[2:]

    elements = []

    for i in range(0, len(children), 2):
        trs = children[i + 1].find('tbody').find_all('tr')
        tds = [str(tr.find_all('td')[0].text) for tr in trs]

        elements.extend(tds)
    elements.remove('<h1>, <h2>, <h3>, <h4>, <h5>, <h6>')
    elements.extend(['<h1>', '<h2>', '<h3>', '<h4>', '<h5>', '<h6>'])

    return elements


def get_elements_with_attrs():
    with open('attrs') as f:
        contents = f.read()

    soup = BeautifulSoup(contents, features="html.parser")

    find = soup.find(class_='standard-table').find('tbody').find_all('tr')

    children = list(find)

    elements = {}

    for element in get_elements():
        elements[element] = {
            'attrs': [],
            'default': None,
        }

    for tr in children:
        tds = list(tr.find_all('td'))
        attr_elements = [e.strip() for e in tds[1].text.split(',')]
        attr_name = tds[0].text.strip()

        if attr_name == 'data-*':
            continue

        for e in attr_elements:
            if e == 'Global attribute':
                for element in elements.keys():
                    elements[element]['attrs'].append(attr_name)
                continue

            try:
                elements[e]['attrs'].append(attr_name)
            except KeyError:
                pass

    for element in elements_with_default:
        elements[element]['default'] = 'child.text'

    return elements
