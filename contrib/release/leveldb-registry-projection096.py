#!/usr/bin/env python3
"""Explicit LevelDB publication-manifest transformation.

The repository's shared Core workspace stays on its selected Git fork.
Apply only to the detached112-member LevelDB package workspace created by
the release controller. Rust sources, package names and features stay
unchanged;102 explicitly derives local versions/requirements at0.2.0. This module performs no network, Cargo, Git or publication action.
"""
import copy
import hashlib
import json
import re
import tomllib

VERSIONS096={'libevent':'0.2.0','libevent-sys':'0.4.0'}
CHECKSUMS096={'libevent':'186cb55cbf50b97003c9e90db596538b5a8ee18cb6737a884031379f30c7a983',
 'libevent-sys':'c3fb4e3d2a502ab90ac5afaa75b502e56bcae710c857833a9675ee17a6e78588'}
FORK096='git+https://github.com/klebs6/libevent-rs?branch=patch-1#25d9732ca6937de48f73a7c78e75ebd35d634ed3'

def manifest096(text,workspace):
    """Replace exactly two version/Git selections, retaining other semantics."""
    before=tomllib.loads(text);path=['workspace','dependencies']if workspace else['dependencies']
    table=before
    for key in path:table=table[key]
    expected=copy.deepcopy(before);target=expected
    for key in path:target=target[key]
    for name in VERSIONS096:
        old=table[name]
        if(old.get('git')!='https://github.com/klebs6/libevent-rs'or old.get('branch')!='patch-1'
                or old.get('version')not in('0.1.0','0.2.5','0.2.6')or set(old)!={'version','git','branch'}):
            raise RuntimeError('Unexpected original native selection: '+name)
        target[name]={'version':'='+VERSIONS096[name]}
    lines=[];section=None;edits=[]
    for line in text.splitlines(keepends=True):
        if line.lstrip().startswith('['):section=line.strip().replace('"','')
        native=next((name for name in VERSIONS096 if section=='['+'.'.join(path+[name])+']'),None)
        key=re.match(r'^\s*"?(version|git|branch)"?\s*=',line)
        if native and key:
            if key[1]=='version':lines.append('version = "='+VERSIONS096[native]+'"\n')
            edits.append((native,key[1]));continue
        lines.append(line)
    after=''.join(lines)
    if tomllib.loads(after)!=expected or len(edits)!=6:raise RuntimeError('Manifest projection exceeded the declared two selections.')
    return after

def lock096(text):
    """Replace the two fork identities in a retained detached workspace lock.

    Every existing registry identity remains fixed. The new wrapper requires
    bitflags2.10; select the already locked compatible2.x identity. Cargo's
    subsequent offline --locked metadata command validates this exact plan.
    """
    before=tomllib.loads(text);expected=copy.deepcopy(before);packages=expected['package']
    flags=[p for p in packages if p['name']=='bitflags'and p['version'].startswith('2.')]
    if len(flags)!=1 or tuple(map(int,flags[0]['version'].split('.'))) < (2,10,0):
        raise RuntimeError('Retained lock lacks one compatible bitflags2 identity.')
    selected={}
    for name,version in VERSIONS096.items():
        rows=[p for p in packages if p['name']==name]
        if len(rows)!=1 or rows[0].get('source')!=FORK096:raise RuntimeError('Retained fork lock identity differs.')
        row=rows[0];row['version']=version;row['source']='registry+https://github.com/rust-lang/crates.io-index';row['checksum']=CHECKSUMS096[name]
        if name=='libevent':
            if row['dependencies']!=['bitflags 1.3.2','libevent-sys']:raise RuntimeError('Retained wrapper dependency graph differs.')
            row['dependencies']=['bitflags '+flags[0]['version'],'libevent-sys']
        elif row['dependencies']!=['bindgen','openssl-sys','pkg-config']:raise RuntimeError('Retained sys dependency graph differs.')
        selected[name]=row
    pieces=re.split(r'(?m)^\[\[package\]\]\n',text);out=pieces[0]
    for piece in pieces[1:]:
        item=tomllib.loads('[[package]]\n'+piece)['package'][0]
        if item['name']not in selected:out+='[[package]]\n'+piece;continue
        row=selected[item['name']];out+='[[package]]\n'
        for key in('name','version','source','checksum'):out+=key+' = '+json.dumps(row[key])+'\n'
        out+='dependencies = '+json.dumps(row['dependencies'])+'\n\n'
    if tomllib.loads(out)!=expected:raise RuntimeError('Lock projection exceeded the two native identities and one wrapper edge.')
    return out

PUBLIC_VERSION102 = '0.2.0'


def sections102(document):
    """Declared dependency tables, including target-specific/dev entries."""
    for prefix,parent in [((),document)]+[(('target',target),row)for target,row in document.get('target',{}).items()]:
        for kind in('dependencies','build-dependencies','dev-dependencies'):
            for alias,value in parent.get(kind,{}).items():
                yield prefix+(kind,alias),alias,({'version':value}if isinstance(value,str)else value)
    for alias,value in document.get('workspace',{}).get('dependencies',{}).items():
        yield('workspace','dependencies',alias),alias,({'version':value}if isinstance(value,str)else value)


def table_path102(line):
    """Parse TOML's quoted table keys instead of splitting cfg strings."""
    parsed=tomllib.loads(line+'\n__release_marker102__ = true\n');path=[];node=parsed
    while isinstance(node,dict)and'__release_marker102__'not in node:
        if len(node)!=1:raise RuntimeError('Ambiguous release manifest table.')
        key,node=next(iter(node.items()));path.append(key)
    if not isinstance(node,dict)or node.get('__release_marker102__')is not True:
        raise RuntimeError('Unsupported release manifest table.')
    return tuple(path)


def inline102(value):
    if isinstance(value,str):return json.dumps(value,ensure_ascii=False)
    if isinstance(value,bool):return'true'if value else'false'
    if isinstance(value,int):return str(value)
    if isinstance(value,list):return'['+', '.join(inline102(x)for x in value)+']'
    if isinstance(value,dict):return'{ '+', '.join(json.dumps(k)+' = '+inline102(v)for k,v in value.items())+' }'
    raise RuntimeError('Unsupported inline release dependency value.')


def versions_manifest102(text,original_versions):
    """Only the selected package identity and existing internal constraints.

    A versionless development fixture remains versionless. Public requirements
    use Cargo's conventional compatible0.2.x range; the verification consumer
    selects exact0.2.0 and an unchanged locked external graph.
    """
    before=tomllib.loads(text);expected=copy.deepcopy(before);wanted={}
    if'package'in before:
        name=before['package']['name']
        if name not in original_versions or before['package']['version']!=original_versions[name]:
            raise RuntimeError('Release package is outside the explicit original-version map.')
        expected['package']['version']=PUBLIC_VERSION102;wanted[('package',)]=PUBLIC_VERSION102
    for path,alias,value in sections102(before):
        name=value.get('package',alias)
        if name not in original_versions or'version'not in value:continue
        target=expected
        for key in path[:-1]:target=target[key]
        if isinstance(target[path[-1]],str):target[path[-1]]=PUBLIC_VERSION102
        else:target[path[-1]]['version']=PUBLIC_VERSION102
        wanted[path]=PUBLIC_VERSION102
    current=();seen=set();lines=[]
    for line in text.splitlines(keepends=True):
        match=re.fullmatch(r'\s*(\[(?!\[).+\])\s*(?:#.*)?\n?',line)
        if match:current=table_path102(match[1])
        key=re.match(r'^\s*version\s*=',line)
        assignment=re.match(r'^(\s*(?:[A-Za-z0-9_-]+|"[^"\n]+"|\'[^\'\n]+\')\s*)=',line)
        alias=None
        if assignment:
            single=tomllib.loads(assignment[1]+'= true')
            if len(single)==1:alias=next(iter(single))
        inline_path=current+(alias,)if alias else None
        if inline_path in wanted:
            target=expected
            for part in inline_path:target=target[part]
            lines.append(assignment[1]+'= '+inline102(target)+'\n');seen.add(inline_path)
        elif key and current in wanted:
            if current in seen:raise RuntimeError('Duplicate projected version field.')
            lines.append('version = "'+wanted[current]+'"\n');seen.add(current)
        else:lines.append(line)
    after=''.join(lines)
    if seen!=set(wanted)or tomllib.loads(after)!=expected:
        raise RuntimeError('Version projection could not express exactly its declared fields; no fallback rewrite.')
    return after


def versions_lock102(text,original_versions,consumer=False):
    """Change only selected local identities and their qualified references."""
    before=tomllib.loads(text)
    if set(before)!={'version','package'}or before['version']!=4:
        raise RuntimeError('Unsupported retained release lock structure.')
    expected=copy.deepcopy(before);locals=[p for p in before['package']if p.get('source')is None]
    if(len({p['name']for p in locals})!=len(locals)
            or len({(p['name'],p['version'],p.get('source'))for p in before['package']})!=len(before['package'])
            or any(p['name']in original_versions for p in before['package']if p.get('source'))):
        raise RuntimeError('Ambiguous duplicate local/registry lock identity.')
    extra={'leveldb-archive-consumer091'}if consumer else set()
    if {p['name']for p in locals}!=set(original_versions)|extra:
        raise RuntimeError('Retained lock local package set differs from the explicit publication set.')
    selected={p['name']:p for p in locals if p['name']in original_versions}
    if len(selected)!=112 or any(p['version']!=original_versions[n]for n,p in selected.items()):
        raise RuntimeError('Retained lock original local versions differ.')
    before_registry={(p['name'],p['version'],p.get('source'),p.get('checksum'))for p in before['package']if p.get('source')}
    for package in expected['package']:
        if package.get('source')is None and package['name']in original_versions:package['version']=PUBLIC_VERSION102
        values=[]
        for label in package.get('dependencies',[]):
            parts=label.split(' ',2)
            if parts[0]in original_versions and len(parts)>1 and len(parts)==2:
                if parts[1]!=original_versions[parts[0]]:raise RuntimeError('Selected local lock reference has an unexpected version.')
                label=parts[0]+' '+PUBLIC_VERSION102
            values.append(label)
        if'dependencies'in package:package['dependencies']=values
    out='# Explicit version-only derivation of the retained release lock.\nversion = 4\n\n'
    for package in expected['package']:
        if not set(package)<={'name','version','source','checksum','dependencies'}:raise RuntimeError('Unsupported retained package-lock fields.')
        out+='[[package]]\n'
        for key in('name','version','source','checksum'):
            if key in package:out+=key+' = '+json.dumps(package[key])+'\n'
        if'dependencies'in package:out+='dependencies = '+json.dumps(package['dependencies'])+'\n'
        out+='\n'
    parsed=tomllib.loads(out)
    if parsed!=expected or {(p['name'],p['version'],p.get('source'),p.get('checksum'))for p in parsed['package']if p.get('source')}!=before_registry:
        raise RuntimeError('Versioned lock altered an external identity or exceeded the declared local edits.')
    if consumer and len(before_registry)!=312:raise RuntimeError('Qualified consumer external registry count differs.')
    return out
