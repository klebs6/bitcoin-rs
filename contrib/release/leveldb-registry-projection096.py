#!/usr/bin/env python3
"""Explicit LevelDB publication-manifest transformation.

The repository's shared Core workspace stays on its selected Git fork.
Apply only to the detached112-member LevelDB package workspace created by
the release controller. Rust sources, package identities and features stay
unchanged. This module performs no network, Cargo, Git or publication action.
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
