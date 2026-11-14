#!/usr/bin/env python3
"""Bounded workspace discovery and selected-package Cargo observation."""
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import stat
import subprocess
import time
import tomllib
from urllib.parse import unquote, urlsplit

_CAT_MAX_PACKAGES055=256
_CAT_MAX_TARGETS055=256
_CAT_MAX_JSON055=16*1024*1024
_OBS_MACHINE055=128*1024*1024
_OBS_RECORD055=1024*1024
_OBS_EVIDENCE055=2*1024*1024
_OBS_SECONDS055=580
_NATIVE_KINDS055=frozenset(('lib','rlib','dylib','cdylib','staticlib','proc-macro','bin','test','example'))
_LIB_KINDS055=frozenset(('lib','rlib','dylib','cdylib','staticlib','proc-macro'))

def _encode055(value):
    return (json.dumps(value,sort_keys=True,indent=2,allow_nan=False)+'\n').encode()

def _pairs055(values):
    result={}
    for key,value in values:
        if key in result:raise RuntimeError('Repeated JSON field')
        result[key]=value
    return result

def _json055(raw):
    def bad(value):raise RuntimeError('Nonfinite JSON value')
    return json.loads(raw,object_pairs_hook=_pairs055,parse_constant=bad)

def _path055(value):
    if not isinstance(value,(str,Path)):raise RuntimeError('Path is not literal text')
    text=str(value);path=Path(text)
    if(not path.is_absolute()or str(path)!=text or '..'in path.parts or len(text.encode())>4096
            or any(ord(c)<32 or ord(c)==127 for c in text)):
        raise RuntimeError('Path is not bounded canonical absolute text')
    return path

def _directory055(path):
    path=_path055(path);fd=os.open('/',os.O_RDONLY|os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child=os.open(part,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW,dir_fd=fd)
            os.close(fd);fd=child
        return fd
    except BaseException:os.close(fd);raise

def _stamp055(info):
    return tuple(getattr(info,k)for k in('st_dev','st_ino','st_mode','st_uid','st_gid','st_nlink','st_size','st_mtime_ns','st_ctime_ns'))

def _file055(path,limit,*,content=True,deadline=None):
    path=_path055(path);parent=_directory055(path.parent);fd=None
    try:
        fd=os.open(path.name,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK,dir_fd=parent);before=os.fstat(fd)
        if(not stat.S_ISREG(before.st_mode)or before.st_uid!=os.getuid()or not 0<=before.st_size<=limit):
            raise RuntimeError('Unowned, nonregular or excessive file: '+path.name)
        digest=hashlib.sha256();chunks=[];count=0
        while True:
            if deadline is not None and time.monotonic()>=deadline:raise RuntimeError('Observer deadline reached while hashing artifact')
            block=os.read(fd,min(65536,limit+1-count))
            if not block:break
            count+=len(block)
            if count>limit:raise RuntimeError('File exceeded finite allowance')
            digest.update(block)
            if content:chunks.append(block)
        after=os.fstat(fd);named=os.stat(path.name,dir_fd=parent,follow_symlinks=False)
        if count!=before.st_size or _stamp055(before)!=_stamp055(after)or _stamp055(before)!=_stamp055(named):
            raise RuntimeError('File changed while inspecting it')
        return {'kind':'file','mode':stat.S_IMODE(before.st_mode),'size':count,'sha256':digest.hexdigest()},b''.join(chunks)
    finally:
        if fd is not None:os.close(fd)
        os.close(parent)

def _features055(package):
    features=package.get('features',{})
    if(type(features)is not dict or len(features)>4096 or any(type(k)is not str or type(v)is not list
            or len(k.encode())>256 or len(v)>4096 or any(type(x)is not str or len(x.encode())>256 for x in v)for k,v in features.items())):
        raise RuntimeError('Package features are malformed or excessive')
    active=set();pending=['default']if'default'in features else[]
    while pending:
        key=pending.pop()
        if key in active:continue
        active.add(key)
        for value in features.get(key,[]):
            if value in features and value not in active:pending.append(value)
            elif '/'in value and '?'not in value.split('/',1)[0]:
                dependency=value.split('/',1)[0]
                if dependency in features and dependency not in active:pending.append(dependency)
    return sorted(active)

def _harness055(manifest,target):
    kinds=set(target['kind']);name=target['name'];source=Path(target['src_path'])
    if kinds&_LIB_KINDS055:
        table=manifest.get('lib',{})
        if type(table)is not dict:raise RuntimeError('Library manifest table is malformed')
        value=table.get('harness',True)
    else:
        category='test'if'test'in kinds else'example'if'example'in kinds else'bin'
        entries=manifest.get(category,[])
        if type(entries)is not list or any(type(v)is not dict for v in entries):raise RuntimeError('Manifest target array is malformed')
        matching=[v for v in entries if v.get('name')==name or v.get('name')is None and type(v.get('path'))is str
                  and Path(target['manifest_path']).parent/v['path']==source]
        if len(matching)>1:raise RuntimeError('Manifest repeats a target binding')
        value=matching[0].get('harness',True)if matching else True
    if type(value)is not bool:raise RuntimeError('Manifest harness selection is not Boolean')
    return value

def _package_identity055(identity,manifest):
    if type(identity)is not str or not identity.startswith('path+')or len(identity)>8192:
        raise RuntimeError('Workspace package has no bounded path identity')
    parsed=urlsplit(identity[5:])
    if(parsed.scheme!='file'or parsed.netloc not in('','localhost')or Path(unquote(parsed.path))!=manifest.parent):
        raise RuntimeError('Package ID differs from its workspace manifest directory')


def metadata_context055(metadata,repo):
    """Validate Cargo metadata using the same16MiB ceiling as its guarded capture."""
    if type(metadata)is not dict:
        raise RuntimeError('Cargo metadata must be an object; observed '+type(metadata).__name__)
    packages=metadata.get('packages');members=metadata.get('workspace_members')
    context={'canonical_bytes':len(_encode055(metadata)),'limit_bytes':_CAT_MAX_JSON055,
        'workspace_root':metadata.get('workspace_root'),'expected_workspace_root':str(repo),
        'version':metadata.get('version'),'packages_type':type(packages).__name__,
        'package_count':len(packages)if type(packages)is list else None,
        'members_type':type(members).__name__,'member_count':len(members)if type(members)is list else None}
    problems=[]
    if context['canonical_bytes']>_CAT_MAX_JSON055:problems.append('metadata exceeds16MiB')
    if metadata.get('workspace_root')!=str(repo):problems.append('workspace_root differs')
    if type(metadata.get('version'))is not int or metadata['version']!=1:problems.append('format version is not1')
    if type(packages)is not list:problems.append('packages is not a list')
    elif len(packages)>4096:problems.append('package count exceeds4096')
    if type(members)is not list:problems.append('workspace_members is not a list')
    elif len(members)>4096:problems.append('member count exceeds4096')
    if problems:
        raise RuntimeError('Cargo metadata context: '+', '.join(problems)+'; '+json.dumps(context,sort_keys=True))
    return context

def discover055(metadata,repo):
    """Return current selected workspace packages, including explicit feature exclusions."""
    repo=_path055(repo);fd=_directory055(repo);os.close(fd)
    metadata_context055(metadata,repo)
    members=metadata['workspace_members']
    if any(type(x)is not str for x in members)or len(members)!=len(set(members)):
        raise RuntimeError('Workspace membership contains malformed or duplicate package IDs')
    result=[];seen_ids=set();seen_names=set()
    for package in metadata['packages']:
        if type(package)is not dict:raise RuntimeError('Metadata package is not an object')
        identity=package.get('id');name=package.get('name')
        if identity not in members:continue
        if type(identity)is not str:raise RuntimeError('Workspace package ID is malformed')
        if type(name)is not str:raise RuntimeError('Workspace package name is malformed')
        if not(name.startswith(('bitcoinleveldb-','bitcoinleveldbt-'))or name=='bitcoin-leveldb'):continue
        if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,127}',name):raise RuntimeError('Selected package name is unsupported')
        if identity in seen_ids or name in seen_names:raise RuntimeError('Selected workspace package identity repeats')
        seen_ids.add(identity);seen_names.add(name)
        if len(result)>=_CAT_MAX_PACKAGES055:raise RuntimeError('Selected package count exceeds256')
        manifest=_path055(package.get('manifest_path'))
        if repo not in manifest.parents or manifest.name!='Cargo.toml':raise RuntimeError('Selected manifest escapes current workspace')
        _package_identity055(identity,manifest)
        manifest_fp,raw=_file055(manifest,1024*1024);document=tomllib.loads(raw.decode('utf8'))
        if type(document.get('package'))is not dict or document['package'].get('name')!=name:
            raise RuntimeError('Live package manifest name differs from Cargo metadata')
        default_features=_features055(package);targets=package.get('targets')
        if type(targets)is not list or len(targets)>_CAT_MAX_TARGETS055:raise RuntimeError('Target count is malformed or excessive')
        selected=[];keys=set()
        for target in targets:
            if(type(target)is not dict or type(target.get('name'))is not str or not target['name'] or len(target['name'].encode())>256
                    or type(target.get('kind'))is not list or not target['kind']
                    or len(target['kind'])>16 or any(type(k)is not str or len(k)>32 for k in target['kind'])or len(set(target['kind']))!=len(target['kind'])
                    or type(target.get('test'))is not bool or type(target.get('doctest'))is not bool):
                raise RuntimeError('Target metadata is malformed')
            source=_path055(target.get('src_path'))
            if repo not in source.parents:raise RuntimeError('Selected target source escapes workspace')
            source_fp,_=_file055(source,16*1024*1024,content=False)
            kinds=sorted(target['kind']);key='|'.join(kinds)+'::'+target['name']
            if key in keys:raise RuntimeError('Selected package repeats a target identity')
            keys.add(key);required=target.get('required-features',[])
            if(type(required)is not list or len(required)>4096 or any(type(x)is not str or len(x.encode())>256 for x in required)
                    or len(set(required))!=len(required)):raise RuntimeError('Required feature list is malformed')
            native=bool(set(kinds)&_NATIVE_KINDS055 and target['test']and'bench'not in kinds)
            selected.append({'key':key,'name':target['name'],'kind':kinds,'src_path':str(source),
                'source_fingerprint':source_fp,'test':target['test'],'doctest':target['doctest'],
                'native_eligible':native,'doctest_eligible':bool(set(kinds)&_LIB_KINDS055 and target['doctest']),
                'harness':_harness055(document,{**target,'manifest_path':str(manifest)})if native else None,
                'required_features':sorted(required),'excluded_by_default_features':bool(set(required)-set(default_features))})
        result.append({'id':identity,'name':name,'manifest_path':str(manifest),'manifest_fingerprint':manifest_fp,
            'default_features_active':default_features,'feature_resolution_scope':'Local default closure is provisional; Cargo artifact features are authoritative when observed.',
            'targets':sorted(selected,key=lambda x:x['key'])})
    if not result:raise RuntimeError('No LevelDB workspace packages were discovered')
    expected={p.get('id')for p in metadata['packages']if type(p)is dict and p.get('id')in members
        and type(p.get('name'))is str and(p['name'].startswith(('bitcoinleveldb-','bitcoinleveldbt-'))or p['name']=='bitcoin-leveldb')}
    if seen_ids!=expected:raise RuntimeError('Selected workspace membership was not fully captured')
    return sorted(result,key=lambda p:p['name'])

# Explicit named Snappy profiles only. Default callers still use the original
# offline, locked, single-job commands without feature flags.
_SNAPPY_FEATURES070 = {
    'bitcoinleveldb-compat': 'leveldb_snappy',
    'bitcoinleveldb-tablebuilder': 'bitcoinleveldb-compat/leveldb_snappy',
    'bitcoinleveldb-blockhandle': 'bitcoinleveldb-compat/leveldb_snappy',
}

def _feature_selection070(value):
    if value is None:
        return {}
    if (type(value) is not dict or len(value) > len(_SNAPPY_FEATURES070)
            or any(type(name) is not str or type(feature) is not str
                   or _SNAPPY_FEATURES070.get(name) != feature
                   for name, feature in value.items())):
        raise RuntimeError('Only the three explicit Snappy package/feature selections are supported.')
    return {name: value[name] for name in sorted(value)}

def _test_command070(name, mode, feature=None):
    if type(name) is not str or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,127}', name):
        raise RuntimeError('The test package name is malformed.')
    command = ['cargo', 'test', '--offline', '--locked', '--jobs', '1', '-p', name]
    if feature is not None:
        if _SNAPPY_FEATURES070.get(name) != feature:
            raise RuntimeError('The requested package feature differs from its explicit Snappy profile.')
        command += ['--features', feature]
    if mode == 'compile':
        return command + ['--no-run', '--message-format=json']
    if mode == 'doc':
        return command + ['--doc', '--', '--include-ignored', '--test-threads=1']
    raise RuntimeError('Only no-run compilation and doctest commands are supported.')

def _planned_test_command070(name, mode, command):
    default = _test_command070(name, mode)
    if command == default:
        return default
    feature = _SNAPPY_FEATURES070.get(name)
    if feature is not None:
        enabled = _test_command070(name, mode, feature)
        if command == enabled:
            return enabled
    raise RuntimeError('The command is neither the exact default nor the explicit Snappy profile.')

def _observer_spec055(spec,spec_path):
    if type(spec)is not dict or set(spec)!={'repo','run','target','package','evidence_path','command'}:
        raise RuntimeError('Observer specification differs from its exact six fields')
    repo,run,target,evidence=(_path055(spec[k])for k in('repo','run','target','evidence_path'))
    stamp=run.parent if run.name=='suite'else run
    if(spec_path.parent!=run or stamp.parent.name!='runs'or stamp.parents[2].name!='codex-repairs'
            or re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}',stamp.name)is None
            or stamp.parents[2]not in target.parents or evidence.parent!=run or evidence.suffix!='.json'
            or re.fullmatch(r'[A-Za-z0-9_.-]{1,160}',evidence.name)is None):
        raise RuntimeError('Observer paths escape their finite current metadata/cache context')
    for path in(repo,run,target):fd=_directory055(path);os.close(fd)
    if os.path.lexists(evidence):raise RuntimeError('Observer evidence path is already occupied')
    package=spec['package']
    if(type(package)is not dict or type(package.get('name'))is not str or type(package.get('targets'))is not list
            or len(package['targets'])>_CAT_MAX_TARGETS055):raise RuntimeError('Observer package catalog is malformed')
    name=package['name']
    if not(name.startswith(('bitcoinleveldb-','bitcoinleveldbt-'))or name=='bitcoin-leveldb')or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,127}',name):
        raise RuntimeError('Observer package is outside the declared LevelDB scope')
    command=_planned_test_command070(name,'compile',spec['command'])
    manifest=_path055(package.get('manifest_path'));_package_identity055(package.get('id'),manifest)
    manifest_fp,manifest_raw=_file055(manifest,1024*1024)
    if (repo not in manifest.parents or manifest_fp!=package.get('manifest_fingerprint')
            or tomllib.loads(manifest_raw.decode('utf8')).get('package',{}).get('name')!=name):
        raise RuntimeError('Observer live package manifest differs from its sealed catalog')
    targets={}
    for item in package['targets']:
        if(type(item)is not dict or type(item.get('key'))is not str or item['key']in targets
                or type(item.get('name'))is not str or not item['name'] or len(item['name'].encode())>256
                or type(item.get('kind'))is not list or not item['kind'] or len(item['kind'])>16
                or any(type(k)is not str or len(k)>32 for k in item['kind'])or len(item['kind'])!=len(set(item['kind']))
                or type(item.get('native_eligible'))is not bool or type(item.get('required_features'))is not list
                or len(item['required_features'])>4096 or any(type(x)is not str or len(x.encode())>256 for x in item['required_features'])
                or type(item.get('test'))is not bool or type(item.get('doctest'))is not bool
                or type(item.get('doctest_eligible'))is not bool
                or (item['native_eligible']and type(item.get('harness'))is not bool)):
            raise RuntimeError('Observer target catalog is malformed')
        source=_path055(item.get('src_path'))
        if repo not in source.parents or _file055(source,16*1024*1024,content=False)[0]!=item.get('source_fingerprint'):
            raise RuntimeError('Observer target source differs from its sealed catalog')
        if item['key']!='|'.join(sorted(item['kind']))+'::'+item['name']:
            raise RuntimeError('Observer target key differs from its metadata')
        targets[item['key']]=item
    return repo,run,target,evidence,package,targets,command

def _save_observer055(path,value):
    raw=_encode055(value)
    if len(raw)>_OBS_EVIDENCE055:raise RuntimeError('Observer evidence exceeds2MiB')
    fd=_directory055(path.parent)
    try:
        stream=os.open(path.name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=fd)
        with os.fdopen(stream,'wb')as out:out.write(raw);out.flush();os.fsync(out.fileno())
        os.fsync(fd)
    finally:os.close(fd)

def observe055(spec_path):
    """Run one exact Cargo no-run invocation; caller supplies the build guard."""
    spec_path=_path055(spec_path);_,raw=_file055(spec_path,2*1024*1024);spec=_json055(raw)
    repo,run,target,evidence_path,package,targets,command=_observer_spec055(spec,spec_path)
    deadline=time.monotonic()+_OBS_SECONDS055
    evidence={'schema':'bitcoinrs-subsystem-cargo-observer055-v1','package':package['name'],'package_id':package['id'],
        'command':command,'repo':str(repo),'run':str(run),'target':str(target),'cargo_exit_code':None,
        'observer_exit_code':2,'outcome':'observer_failed','build_finished':None,'artifacts':{},
        'machine_bytes':0,'stderr_bytes':0,'records':0,'dependency_artifacts_discarded':0,
        'non_test_artifacts_discarded':0,'diagnostic_counts':{},'errors':[],'warnings':[],'diagnostics_omitted':0,
        'stderr_tail':'','invalid_record_prefix':None,'observed_features':None,'effective_features_basis':None,'excluded_targets':[],'error':None}
    child=None;selector=selectors.DefaultSelector();pending=bytearray();features=[];stderr_tail=bytearray()
    def check():
        if time.monotonic()>=deadline:raise RuntimeError('Finite Cargo observer deadline reached')
    def record(line):
        check();evidence['records']+=1
        if evidence['records']>200000:raise RuntimeError('Cargo record allowance exceeded')
        try:value=_json055(line)
        except BaseException:
            evidence['invalid_record_prefix']=line[:4096].decode('utf8','replace')
            raise
        if type(value)is not dict:raise RuntimeError('Cargo machine record is not an object')
        reason=value.get('reason')
        if reason=='build-finished':
            if type(value.get('success'))is not bool or evidence['build_finished']is not None:
                raise RuntimeError('Cargo build-finished record is malformed or repeated')
            evidence['build_finished']=value['success'];return
        if reason=='compiler-message':
            message=value.get('message',{});level=message.get('level','unknown')
            if type(level)is not str:raise RuntimeError('Cargo diagnostic level is malformed')
            if level not in('error','warning','note','help','failure-note','ice'):level='unknown'
            counts=evidence['diagnostic_counts'];counts[level]=counts.get(level,0)+1
            if level in('error','failure-note')or level=='warning':
                key='errors'if level in('error','failure-note')else'warnings';cap=32 if key=='errors'else 4
                direct=key=='errors'and value.get('package_id')==package['id']
                if direct and len(evidence[key])>=cap:
                    indirect=next((i for i,v in enumerate(evidence[key])if v['package_id']!=package['id']),None)
                    if indirect is not None:
                        evidence[key].pop(indirect);evidence['diagnostics_omitted']+=1
                if len(evidence[key])<cap:
                    candidate={'package_id':str(value.get('package_id',''))[:8192],
                        'level':level,'message':str(message.get('message',''))[:4096],
                        'rendered':str(message.get('rendered')or'')[:8192]}
                    while key=='errors'and evidence['warnings']and len(_encode055(evidence['errors']+evidence['warnings']+[candidate]))>512*1024:
                        evidence['warnings'].pop();evidence['diagnostics_omitted']+=1
                    while direct and len(_encode055(evidence['errors']+evidence['warnings']+[candidate]))>512*1024:
                        indirect=next((i for i,v in enumerate(evidence['errors'])if v['package_id']!=package['id']),None)
                        if indirect is None:break
                        evidence['errors'].pop(indirect);evidence['diagnostics_omitted']+=1
                    if len(_encode055(evidence['errors']+evidence['warnings']+[candidate]))<=512*1024:
                        if direct:evidence[key].insert(0,candidate)
                        else:evidence[key].append(candidate)
                    else:evidence['diagnostics_omitted']+=1
                else:evidence['diagnostics_omitted']+=1
            return
        if reason!='compiler-artifact':return
        if value.get('package_id')!=package['id']:
            evidence['dependency_artifacts_discarded']+=1;return
        item=value.get('target');profile=value.get('profile')
        if type(item)is not dict or type(profile)is not dict:raise RuntimeError('Selected Cargo artifact lacks target/profile data')
        kinds=item.get('kind')
        if (type(kinds)is not list or not kinds or len(kinds)>16 or any(type(x)is not str or len(x)>32 for x in kinds)
                or len(kinds)!=len(set(kinds))):raise RuntimeError('Artifact target kinds are malformed')
        key='|'.join(sorted(kinds))+'::'+str(item.get('name'))
        if key not in targets:raise RuntimeError('Cargo emitted an unrecorded selected-package target')
        expected=targets[key]
        if item.get('src_path')!=expected['src_path']:raise RuntimeError('Cargo artifact source differs from the current catalog')
        observed=value.get('features')
        if(type(observed)is not list or len(observed)>4096 or any(type(x)is not str or len(x.encode())>256 for x in observed)
                or len(set(observed))!=len(observed) or len(_encode055(observed))>32768):
            raise RuntimeError('Cargo artifact feature list is malformed')
        if 'custom-build'not in kinds:
            normalized=sorted(observed)
            if features and normalized!=features[0]:raise RuntimeError('Selected Cargo targets disagree on effective features')
            if not features:features.append(normalized)
        if type(profile.get('test'))is not bool:raise RuntimeError('Artifact profile.test is not Boolean')
        if profile.get('test')is not True or value.get('executable')is None:
            evidence['non_test_artifacts_discarded']+=1;return
        if expected.get('native_eligible')is not True:raise RuntimeError('Cargo emitted an unexpected executable test target')
        path=_path055(value['executable'])
        if path.parent not in(target/'debug/deps',target/'debug/examples'):
            raise RuntimeError('Test executable escapes the selected debug artifact cache')
        if key in evidence['artifacts']or any(x['path']==str(path)for x in evidence['artifacts'].values()):
            raise RuntimeError('Cargo repeated an executable target or path')
        fp,_=_file055(path,256*1024*1024,content=False,deadline=deadline)
        if not fp['mode']&0o111 or fp['size']==0:raise RuntimeError('Cargo test artifact is not executable')
        evidence['artifacts'][key]={'path':str(path),'fingerprint':fp,'target':expected,'package_id':package['id'],
            'features':sorted(set(observed))}
        if len(_encode055(evidence))>_OBS_EVIDENCE055-768*1024:
            del evidence['artifacts'][key]
            raise RuntimeError('Artifact catalog exceeds the finite evidence allowance')
    try:
        env=dict(os.environ,CARGO_TARGET_DIR=str(target),CARGO_BUILD_JOBS='1',CARGO_INCREMENTAL='0',RUST_LOG='off')
        child=subprocess.Popen(command,cwd=repo,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        for stream,label in((child.stdout,'stdout'),(child.stderr,'stderr')):
            os.set_blocking(stream.fileno(),False);selector.register(stream,selectors.EVENT_READ,label)
        while selector.get_map():
            check()
            for key,_ in selector.select(min(0.2,max(0,deadline-time.monotonic()))):
                chunk=os.read(key.fileobj.fileno(),65536)
                if not chunk:
                    selector.unregister(key.fileobj)
                    if key.data=='stdout'and pending:record(bytes(pending));pending.clear()
                    continue
                if key.data=='stderr':
                    evidence['stderr_bytes']+=len(chunk);stderr_tail.extend(chunk);del stderr_tail[:-16384]
                else:
                    evidence['machine_bytes']+=len(chunk);pending.extend(chunk)
                    while b'\n'in pending:
                        line,_,remaining=pending.partition(b'\n');pending=bytearray(remaining)
                        if len(line)>_OBS_RECORD055:raise RuntimeError('Cargo record exceeds1MiB')
                        if line.strip():record(bytes(line))
                    if len(pending)>_OBS_RECORD055:raise RuntimeError('Cargo record exceeds1MiB')
                if evidence['machine_bytes']+evidence['stderr_bytes']>_OBS_MACHINE055:
                    raise RuntimeError('Cargo aggregate output exceeds128MiB')
        evidence['cargo_exit_code']=child.wait(timeout=max(0.01,deadline-time.monotonic()))
        if features and any(x!=features[0]for x in features):raise RuntimeError('Selected Cargo targets disagree on effective features')
        active=features[0]if features else package.get('default_features_active',[])
        evidence['observed_features']=active;evidence['effective_features_basis']='cargo-artifact'if features else'metadata-local-default-closure'
        expected_keys=set()
        for key,item in targets.items():
            if not item['native_eligible']:continue
            if set(item['required_features'])-set(active):
                evidence['excluded_targets'].append({'key':key,'reason':'required_features_inactive','required_features':item['required_features']})
            else:expected_keys.add(key)
        if evidence['cargo_exit_code']==0:
            if evidence['build_finished']is not True:raise RuntimeError('Cargo exited zero without a successful build-finished record')
            if set(evidence['artifacts'])!=expected_keys:raise RuntimeError('Cargo executable inventory is missing or unexpected')
            evidence.update(outcome='compiled'if expected_keys else'compiled_no_native_targets',observer_exit_code=0)
        elif evidence['cargo_exit_code']==101:
            if evidence['build_finished']is not False:raise RuntimeError('Cargo101 lacks a failed build-finished record')
            evidence.update(outcome='compile_failed',observer_exit_code=101)
        else:raise RuntimeError('Cargo exited with an unsupported status')
    except BaseException as error:
        evidence['error']=type(error).__name__+': '+str(error)[:8192]
        if child is not None and child.poll()is None:
            child.terminate()
            try:child.wait(timeout=1)
            except subprocess.TimeoutExpired:child.kill();child.wait(timeout=1)
        if child is not None:evidence['cargo_exit_code']=child.returncode
    finally:
        selector.close()
        if child is not None:
            for stream in(child.stdout,child.stderr):
                if stream is not None:stream.close()
        evidence['stderr_tail']=bytes(stderr_tail).decode('utf8','replace')
        _save_observer055(evidence_path,evidence)
    print('SUBSYSTEM055_CARGO '+json.dumps({'package':package['name'],'outcome':evidence['outcome'],
        'native_exit':evidence['cargo_exit_code'],'artifacts':len(evidence['artifacts'])}),flush=True)
    return evidence['observer_exit_code']

compile_observer055=observe055


# The discovery and Cargo observer functions above share this readable module.
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import selectors
import shlex
import shutil
import signal
import stat
import subprocess
import sys
import time
import types
import uuid

sys.dont_write_bytecode = True
MIB055 = 1024 * 1024
WORKFLOW055 = 'leveldb-subsystem055'
SUPPORT_SHA055 = 'ebe9a5b35a13b5a5ebe43b7561c14df272b4b4b2f1ae0ecfcb20bf911c48e2ca'
SCRIPT055 = Path(__file__).absolute()
TOOL_PATHS055 = ('contrib/test/leveldb-subsystem.py', 'bitcoin-leveldb/check.py')
_SUPPORT055 = None


def support055():
    global _SUPPORT055
    if _SUPPORT055 is not None:
        return _SUPPORT055
    path = SCRIPT055.with_name('leveldb-final-crates.py')
    _, raw = _file055(path, 256 * 1024)
    if hashlib.sha256(raw).hexdigest() != SUPPORT_SHA055:
        raise RuntimeError('The sibling repaired-case support file differs from its reviewed bytes.')
    mod = types.ModuleType('leveldb_subsystem_support033')
    mod.__file__ = str(path)
    sys.modules[mod.__name__] = mod
    exec(compile(raw, str(path), 'exec'), mod.__dict__)
    _SUPPORT055 = mod
    return mod


def canonical055(value):
    return (json.dumps(value, sort_keys=True, indent=2, allow_nan=False) + '\n').encode()


def equal055(a, b):
    return canonical055(a) == canonical055(b)


def json055(path, value, limit=32 * MIB055):
    raw = canonical055(value)
    if len(raw) > limit:
        raise RuntimeError('The bounded subsystem report allowance was reached.')
    path = Path(path)
    fd = support055().ordinary_dir(path.parent)
    leaf = '.' + path.name + '-' + uuid.uuid4().hex
    try:
        try:
            old = os.stat(path.name, dir_fd=fd, follow_symlinks=False)
            if not stat.S_ISREG(old.st_mode) or old.st_uid != os.getuid():
                raise RuntimeError('Output destination is not an ordinary owned file.')
        except FileNotFoundError:
            pass
        out = os.open(leaf, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd)
        with os.fdopen(out, 'wb') as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(leaf, path.name, src_dir_fd=fd, dst_dir_fd=fd)
        os.fsync(fd)
    finally:
        try:
            os.unlink(leaf, dir_fd=fd)
        except FileNotFoundError:
            pass
        os.close(fd)


def read_json055(path, limit=16 * MIB055):
    fp, raw = support055().read_file033(Path(path), limit)
    return json.loads(raw), fp


def bounded_command055(command, cwd, env, seconds=580, allowance=16 * MIB055):
    """Capture finite command output; no shell, retry, or uncapped pipe."""
    child = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    streams = {child.stdout: bytearray(), child.stderr: bytearray()}
    selector = selectors.DefaultSelector()
    deadline = time.monotonic() + seconds
    try:
        for stream in streams:
            os.set_blocking(stream.fileno(), False)
            selector.register(stream, selectors.EVENT_READ)
        while selector.get_map():
            if time.monotonic() > deadline:
                raise RuntimeError('The bounded subprocess deadline was reached.')
            for event, _ in selector.select(0.1):
                chunk = os.read(event.fileobj.fileno(), 65536)
                if not chunk:
                    selector.unregister(event.fileobj)
                    continue
                streams[event.fileobj].extend(chunk)
                if sum(map(len, streams.values())) > allowance:
                    raise RuntimeError('The bounded subprocess output allowance was reached.')
        code = child.wait(timeout=max(0.1, deadline - time.monotonic()))
        return code, bytes(streams[child.stdout]), bytes(streams[child.stderr])
    finally:
        if child.poll() is None:
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            child.wait(timeout=2)
        selector.close()
        for stream in streams:
            stream.close()


def snapshot_worker055(repo, baseline_path, output):
    base = support055()
    value = {'schema': 'bitcoinrs-subsystem-inputs055-v1',
             'working_code': base.capture_inputs033(Path(repo)),
             'tooling': {name: base.read_file033(Path(repo) / name, 2 * MIB055, False)[0]
                         for name in TOOL_PATHS055}}
    digest = hashlib.sha256(canonical055(value)).hexdigest()
    if baseline_path == '-':
        json055(output, value, 16 * MIB055)
        print('SUBSYSTEM055_INITIAL_INPUTS ' + digest)
        return 0
    baseline, fp = read_json055(baseline_path)
    same = equal055(value, baseline)
    proof = {'schema': 'bitcoinrs-subsystem-input-proof055-v1', 'baseline': baseline_path,
             'baseline_fingerprint': fp, 'canonical_sha256': digest, 'inputs_preserved': same}
    if not same:
        changed = str(Path(output).with_suffix('.observed.json'))
        json055(changed, value, 16 * MIB055)
        proof['observed_file'] = changed
    json055(output, proof)
    print('SUBSYSTEM055_INPUTS ' + ('PRESERVED' if same else 'CHANGED'))
    return 0 if same else 3


def metadata_worker055(repo, output):
    env = support055().checked_environment033()
    code, out, err = bounded_command055(
        ['cargo', 'metadata', '--offline', '--locked', '--no-deps', '--format-version', '1'],
        repo, env, seconds=170)
    if code:
        print(err[-32768:].decode(errors='replace'), file=sys.stderr)
        return code
    value = json.loads(out)
    json055(output, value, 16 * MIB055)
    print('SUBSYSTEM055_METADATA_RECORDED')
    return 0


SUMMARY055 = re.compile(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; '
                        r'(\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+$', re.M)


def doc_worker055(spec_path):
    spec, _ = read_json055(spec_path, 2 * MIB055)
    command = _planned_test_command070(spec['package'], 'doc', spec['command'])
    env = support055().checked_environment033()
    code, out, err = bounded_command055(command, spec['repo'], env, allowance=16 * MIB055)
    text = out.decode(errors='replace')
    summaries = [dict(status=m[0], passed=int(m[1]), failed=int(m[2]), ignored=int(m[3]),
                      measured=int(m[4]), filtered=int(m[5])) for m in SUMMARY055.findall(text)]
    summary_lines = [line for line in text.splitlines() if line.startswith('test result:')]
    passed = code == 0 and bool(summaries) and len(summaries) == len(summary_lines) and all(
        x['status'] == 'ok' and x['failed'] == x['ignored'] == x['measured'] == x['filtered'] == 0
        for x in summaries)
    value = {'schema': 'bitcoinrs-subsystem-doctests055-v1', 'command': command,
             'repo': spec['repo'], 'package': spec['package'], 'cargo_exit_code': code,
             'outcome': 'passed' if passed else 'failed', 'summaries': summaries,
             'passed': sum(x['passed'] for x in summaries),
             'stdout_tail': text[-65536:], 'stderr_tail': err[-65536:].decode(errors='replace'),
             'stdout_bytes': len(out), 'stderr_bytes': len(err)}
    json055(spec['evidence_path'], value, 2 * MIB055)
    print('SUBSYSTEM055_DOCTESTS ' + value['outcome'])
    return 0 if passed else 1


def binding055(base, guard_module, run, row, command, repo, env, kind, artifact=None):
    """Bind real guard controls and recorded execution to the expected command."""
    budget = dict(guard_module.BUDGETS[kind])
    log = Path(row.get('log', ''))
    group = log.parent
    normalized = [os.path.abspath(shutil.which(command[0], path=base.checked_environment033().get('PATH'))
                                  or command[0]), *map(str, command[1:])]
    if (row.get('verified') is not True or row.get('kind') != kind or row.get('command') != normalized
            or row.get('cwd') != str(repo) or not equal055(row.get('budget'), budget)
            or log.name != 'output.log' or group.parent != run / 'guard'):
        raise RuntimeError('The guard command, location, kind, or controls differ from the recorded plan.')
    plan, _ = read_json055(group / 'plan.json', MIB055)
    probe, _ = read_json055(group / 'probe.json', MIB055)
    result, _ = read_json055(group / 'result.json', MIB055)
    permit_fp, permit = base.read_file033(group / 'permit', MIB055)
    log_fp, raw = base.read_file033(log, budget['output'])
    if (not equal055(row, result) or not equal055(probe, row.get('probe'))
            or plan.get('command') != normalized or plan.get('cwd') != str(repo)
            or plan.get('kind') != kind or permit != str(plan.get('nonce', '')).encode()
            or not re.fullmatch('[0-9a-f]{32}', plan.get('nonce', ''))
            or plan.get('probe') != str(group / 'probe.json')
            or plan.get('permit') != str(group / 'permit')
            or any(plan.get('env', {}).get(key) != str(value) for key, value in env.items())
            or type(row.get('output_bytes')) is not int or row['output_bytes'] < 0
            or min(row['output_bytes'], budget['output']) != len(raw)):
        raise RuntimeError('The guard plan, environment, permit, result, or output binding differs.')
    base._kernel033(probe, budget)
    if artifact is not None:
        path = Path(artifact['path'])
        if base.read_file033(path, 256 * MIB055, False)[0] != artifact['fingerprint']:
            raise RuntimeError('The selected native executable changed after compilation.')
    result_fp = base.read_file033(group / 'result.json', MIB055, False)[0]
    return {'log_fingerprint': log_fp, 'permit_fingerprint': permit_fp, 'result_fingerprint': result_fp,
            'command_bound': True, 'kernel_controls_verified': True}, raw.decode(errors='replace')


def lock_roots055(base, common, own):
    root = common / 'codex-repairs'
    root.mkdir(mode=0o700, exist_ok=True)
    own.mkdir(mode=0o700, exist_ok=True)
    paths = sorted(x / 'run.lock' for x in root.iterdir() if x.is_dir() and not x.is_symlink())
    if len(paths) > 512:
        raise RuntimeError('Too many workflow locks for the bounded subsystem runner.')
    result = []
    try:
        for path in paths:
            parent = base.ordinary_dir(path.parent)
            try:
                fd = os.open(path.name, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600, dir_fd=parent)
            finally:
                os.close(parent)
            try:
                if not stat.S_ISREG(os.fstat(fd).st_mode):
                    raise RuntimeError('Workflow lock is not an ordinary file.')
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                result.append(fd)
            except BaseException:
                os.close(fd)
                raise
        return result
    except BaseException:
        for fd in result:
            os.close(fd)
        raise


def select_packages055(packages, names):
    by_name = {p['name']: p for p in packages}
    if len(by_name) != len(packages):
        raise RuntimeError('The selected workspace contains duplicate package names.')
    if names:
        if len(set(names)) != len(names) or any(name not in by_name for name in names):
            raise RuntimeError('Requested package names must be unique discovered LevelDB workspace members.')
        packages = [p for p in packages if p['name'] in set(names)]
    if not packages:
        raise RuntimeError('No LevelDB workspace members were selected.')
    return sorted(packages, key=lambda p: p['name'])


def known_case055(base, package, target, name):
    values = [dict(x) for x in base.CASES033
              if base.TARGETS033[x['crate']]['package'] == package
              and x['target'] == target and x['name'] == name]
    if len(values) > 1:
        raise RuntimeError('A repaired exact test has more than one catalog binding.')
    return values[0] if values else None


def qualify_observer055(spec, evidence, row):
    """Qualify a fresh dynamic inventory without changing the fixed repaired catalog."""
    if (not isinstance(evidence, dict) or row.get('outcome') != 'exited' or row.get('exit_code') != 0
            or evidence.get('schema') != 'bitcoinrs-subsystem-cargo-observer055-v1'
            or evidence.get('outcome') not in {'compiled', 'compiled_no_native_targets'}
            or evidence.get('cargo_exit_code') != 0 or evidence.get('observer_exit_code') != 0
            or evidence.get('build_finished') is not True
            or evidence.get('package') != spec['package']['name']
            or evidence.get('package_id') != spec['package']['id']
            or any(evidence.get(key) != spec[key] for key in ('repo', 'run', 'target', 'command'))):
        return None
    active = evidence.get('observed_features')
    if not isinstance(active, list) or any(not isinstance(x, str) for x in active) or len(set(active)) != len(active):
        raise RuntimeError('Cargo feature evidence is malformed.')
    expected, excluded = {}, []
    for item in spec['package']['targets']:
        if not item['native_eligible']:
            continue
        if set(item['required_features']) - set(active):
            excluded.append({'key': item['key'], 'reason': 'required_features_inactive',
                             'required_features': item['required_features']})
        else:
            expected[item['key']] = item
    artifacts = evidence.get('artifacts')
    if not isinstance(artifacts, dict) or set(artifacts) != set(expected) or evidence.get('excluded_targets') != excluded:
        raise RuntimeError('Fresh Cargo executable coverage differs from its package/feature inventory.')
    paths = set()
    base = support055()
    for key, artifact in artifacts.items():
        if (not isinstance(artifact, dict) or artifact.get('package_id') != spec['package']['id']
                or not equal055(artifact.get('target'), expected[key]) or artifact.get('features') != sorted(active)):
            raise RuntimeError('A fresh executable differs from its exact selected package/target.')
        path = Path(artifact.get('path', ''))
        target = Path(spec['target'])
        if (not path.is_absolute() or '..' in path.parts or path in paths
                or path.parent not in {target / 'debug/deps', target / 'debug/examples'}):
            raise RuntimeError('A fresh native executable escapes the declared target cache.')
        paths.add(path)
        fp = base.read_file033(path, 256 * MIB055, False)[0]
        if fp != artifact.get('fingerprint') or not fp['mode'] & 0o111 or not fp['size']:
            raise RuntimeError('Fresh executable bytes or executable mode changed.')
    return artifacts


def qualify_doc055(spec, value, row):
    if (not isinstance(value, dict) or value.get('schema') != 'bitcoinrs-subsystem-doctests055-v1'
            or value.get('outcome') != 'passed' or type(value.get('cargo_exit_code')) is not int
            or value['cargo_exit_code'] != 0 or row.get('outcome') != 'exited' or row.get('exit_code') != 0
            or any(value.get(key) != spec[key] for key in ('repo', 'package', 'command'))):
        return False
    summaries = value.get('summaries')
    if not isinstance(summaries, list) or not summaries:
        return False
    for item in summaries:
        if (not isinstance(item, dict) or item.get('status') != 'ok'
                or any(type(item.get(k)) is not int or item[k] < 0 for k in ('passed', 'failed', 'ignored', 'measured', 'filtered'))
                or any(item[k] != 0 for k in ('failed', 'ignored', 'measured', 'filtered'))):
            return False
    return type(value.get('passed')) is int and value['passed'] == sum(item['passed'] for item in summaries)


def failure_sources055(base, repo, package, allowance):
    """Retain bounded failed-package source so repair needs no collection round."""
    root = Path(package['manifest_path']).parent
    result = {'files': [], 'omissions': [], 'bytes': 0}
    deadline = time.monotonic() + 10
    for directory, dirs, names in os.walk(root, followlinks=False):
        dirs[:] = sorted(d for d in dirs if d not in {'target', '.git'}
                         and not Path(directory, d).is_symlink())
        for name in sorted(names):
            path = Path(directory, name)
            if path.suffix != '.rs' and name != 'Cargo.toml':
                continue
            relative = str(path.relative_to(repo))
            if time.monotonic() > deadline or len(result['files']) >= 128 or result['bytes'] >= allowance:
                result['omissions'].append(relative)
                if len(result['omissions']) >= 128:
                    result['omissions'].append('further source paths omitted at bounded allowance')
                    return result
                continue
            try:
                fp, raw = base.read_file033(path, min(256 * 1024, allowance - result['bytes']))
                result['files'].append({'path': relative, 'fingerprint': fp,
                                        'content': raw.decode('utf-8', errors='replace')})
                result['bytes'] += len(raw)
            except Exception as error:
                result['omissions'].append(relative + ': ' + str(error)[:200])
    return result



EXPLICIT_STRESS057 = {
    ('bitcoinleveldbt-issue178', 'bitcoinleveldbt_issue178::issue178_test'):
        'ISSUE178_CONTRACT_OK case=issue178_test',
    ('bitcoinleveldbt-sparsemerge', 'bitcoinleveldbt_sparsemerge::db_test_sparse_merge'):
        'SPARSEMERGE_CONTRACT_OK case=db_test_sparse_merge',
    # Each option configuration includes an unchanged ten-second workload.
    # This original libtest case has no additional in-body marker.
    ('bitcoinleveldbt-mt', 'mt_test::db_test_multi_threaded'): None,
}


def assess_case057(result, log, expected_name, marker=None):
    """Accept libtest's expected-panic suffix without relaxing exact-case proof."""
    assessment = {'outcome': 'failed', 'expected_name': expected_name, 'passed': None,
                  'failed': None, 'ignored': None, 'marker': marker, 'error': None,
                  'parser': 'exact-libtest-should-panic057'}
    try:
        if (not isinstance(result, dict) or result.get('verified') is not True
                or result.get('outcome') != 'exited' or type(result.get('exit_code')) is not int
                or result['exit_code'] != 0):
            raise RuntimeError('Selected case did not exit successfully under verified controls.')
        if (not isinstance(log, str) or not isinstance(expected_name, str) or not expected_name
                or re.search(r'\s', expected_name)):
            raise RuntimeError('Selected case log or expected name is malformed.')
        heads = re.findall(r'(?m)^test ([^\s]+)(?: - should panic)? \.\.\.(?:[ \t]|$)', log)
        if heads != [expected_name]:
            raise RuntimeError('Selected case log does not contain exactly the requested test name.')
        summaries = [line for line in log.splitlines() if line.startswith('test result:')]
        if len(summaries) != 1:
            raise RuntimeError('Selected case log lacks one unambiguous test-result summary.')
        match = re.fullmatch(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; '
                             r'(\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+', summaries[0])
        if match is None:
            raise RuntimeError('Selected case summary is malformed.')
        passed, failed, ignored, measured = map(int, match.group(2, 3, 4, 5))
        assessment.update(passed=passed, failed=failed, ignored=ignored)
        if match.group(1) != 'ok' or (passed, failed, ignored, measured) != (1, 0, 0, 0):
            raise RuntimeError('Selected case did not report exactly one passing non-ignored test.')
        if marker is not None:
            if not isinstance(marker, str) or not marker or '\n' in marker:
                raise RuntimeError('Required case marker is malformed.')
            if len(re.findall(r'(?<!\S)' + re.escape(marker) + r'(?!\S)', log)) != 1:
                raise RuntimeError('Selected case required marker is absent or repeated.')
        assessment['outcome'] = 'passed'
    except Exception as error:
        assessment['error'] = str(error)[:1024]
    return assessment


"""Retained history and pending-check selection; no historical command execution."""

def _successful_row058(report, label):
    row = report.get('readouts', {}).get(label, {})
    proof = report.get('execution_bindings', {}).get(label, {})
    return (isinstance(row, dict) and row.get('verified') is True
            and row.get('outcome') == 'exited' and type(row.get('exit_code')) is int
            and row['exit_code'] == 0 and proof.get('command_bound') is True
            and proof.get('kernel_controls_verified') is True)


def _history_token058(value):
    return isinstance(value, str) and bool(value) and len(value.encode()) <= 8192 and not re.search(r'\s', value)


def empty_history058():
    return {'compiled': set(), 'lists': {}, 'passed': {}, 'docs': {}, 'customs': {},
            'sources': [], 'workspace_packages': []}


def history_from_report058(value, source):
    """Read a generated receipt into compact historical identities, never fresh counts."""
    if not isinstance(value, dict):
        raise RuntimeError('Focus baseline must be a structured report.')
    report = value.get('verification', value)
    if not isinstance(report, dict) or not isinstance(report.get('repository'), str):
        raise RuntimeError('Focus baseline lacks its working-tree report.')
    feature_selection = _feature_selection070(report.get('feature_selection070'))
    history = empty_history058()
    state = report.get('continuation_state058')
    if state is None and report.get('run_mode058') == 'failed-and-unexecuted':
        raise RuntimeError('Focused receipt lacks continuation history; select its earlier retained baseline.')
    if state is not None:
        if (not isinstance(state, dict) or state.get('schema') != 'bitcoinrs-subsystem-continuation058-v1'
                or state.get('repository') != report['repository']):
            raise RuntimeError('Focus continuation state has an unsupported schema/context.')
        references = state.get('sources')
        if not isinstance(references, list) or len(references) > 1024 or any(not isinstance(x, dict) for x in references):
            raise RuntimeError('Focus history references are malformed or excessive.')
        for reference in references:
            if reference.get('kind') == 'current_run':
                if reference.get('run_directory') != report.get('run_directory'):
                    raise RuntimeError('Current-run history reference differs from the baseline receipt.')
                reference = dict(source, kind='retained_report')
            elif reference.get('kind') != 'retained_report':
                raise RuntimeError('Unsupported focus history reference kind.')
            history['sources'].append(dict(reference))
        compiled = state.get('compiled_packages')
        if (not isinstance(compiled, list) or len(compiled) > 256
                or any(not _history_token058(x) for x in compiled) or len(compiled) != len(set(compiled))):
            raise RuntimeError('Focus compiled-package history is malformed.')
        history['compiled'] = set(compiled)
        lists = state.get('target_lists')
        if not isinstance(lists, dict) or len(lists) > 65536:
            raise RuntimeError('Focus test-list history is malformed.')
        total = 0
        for key, record in lists.items():
            if (not _history_token058(key) or not isinstance(record, dict)
                    or not isinstance(record.get('all'), list) or not isinstance(record.get('ignored'), list)):
                raise RuntimeError('Focus target-list record is malformed.')
            names, ignored = record['all'], record['ignored']
            total += len(names)
            if (total > 20000 or any(not _history_token058(x) for x in names + ignored)
                    or len(names) != len(set(names)) or len(ignored) != len(set(ignored))
                    or not set(ignored).issubset(names)):
                raise RuntimeError('Focus target-list identities are invalid or excessive.')
            history['lists'][key] = {'all': list(names), 'ignored': list(ignored)}
        for field, width in (('passed_native', 3), ('passed_doctests', 1), ('passed_customs', 2)):
            records = state.get(field)
            if not isinstance(records, list) or len(records) > 20000:
                raise RuntimeError('Focus pass history is malformed or excessive.')
            destination = history[{'passed_native': 'passed', 'passed_doctests': 'docs', 'passed_customs': 'customs'}[field]]
            for record in records:
                if (not isinstance(record, list) or len(record) != width + 1
                        or any(not _history_token058(x) for x in record[:-1])
                        or type(record[-1]) is not int or not 0 <= record[-1] < len(history['sources'])):
                    raise RuntimeError('Focus pass/source identity is malformed.')
                identity = record[0] if width == 1 else tuple(record[:-1])
                if identity in destination:
                    raise RuntimeError('Focus history repeats a pass identity.')
                destination[identity] = record[-1]
        history['workspace_packages'] = state.get('workspace_packages', [])
        if not isinstance(history['workspace_packages'], list) or len(history['workspace_packages']) > 256:
            raise RuntimeError('Focus workspace catalog is malformed.')
        return history, report

    history['sources'] = [dict(source, kind='retained_report')]
    history['workspace_packages'] = report.get('workspace_packages', report.get('packages', []))
    if not isinstance(history['workspace_packages'], list) or len(history['workspace_packages']) > 256:
        raise RuntimeError('Focus baseline package catalog is malformed.')
    compilations = report.get('compilations', {})
    if not isinstance(compilations, dict):
        raise RuntimeError('Focus baseline compilation records are malformed.')
    for name, compilation in compilations.items():
        evidence = compilation.get('evidence', {}) if isinstance(compilation, dict) else {}
        if (evidence.get('outcome') in {'compiled', 'compiled_no_native_targets'}
                and _successful_row058(report, compilation.get('guard_label'))):
            history['compiled'].add(name)
    lists = report.get('test_lists', {})
    if not isinstance(lists, dict):
        raise RuntimeError('Focus baseline test lists are malformed.')
    history['lists'] = lists.copy()
    for case in report.get('cases', []):
        assessment = case.get('assessment', {}) if isinstance(case, dict) else {}
        if assessment.get('outcome') != 'passed':
            continue
        if (assessment.get('passed'), assessment.get('failed'), assessment.get('ignored')) != (1, 0, 0):
            raise RuntimeError('Focus baseline claims a pass without one exact successful test.')
        package, name, label = case.get('package'), case.get('name'), case.get('guard_label')
        if not _successful_row058(report, label):
            raise RuntimeError('Focus baseline pass lacks successful verified execution evidence.')
        evidence = compilations.get(package, {}).get('evidence', {})
        artifacts = evidence.get('artifacts', {})
        matches = [(key, artifact) for key, artifact in artifacts.items()
                   if artifact.get('target', {}).get('name') == case.get('target')
                   and equal055(artifact.get('fingerprint'), case.get('artifact_fingerprint'))]
        if len(matches) != 1:
            raise RuntimeError('Focus baseline pass cannot be bound to one historical Cargo target/artifact.')
        target, artifact = matches[0]
        row = report['readouts'][label]
        if (row.get('command', [])[:3] != [artifact['path'], '--exact', name]
                or assessment.get('expected_name') != name
                or name not in lists.get(package + '::' + target, {}).get('all', [])):
            raise RuntimeError('Focus baseline pass differs from its exact selected test/list.')
        identity = (package, target, name)
        if identity in history['passed']:
            raise RuntimeError('Focus baseline repeats a native pass identity.')
        history['passed'][identity] = 0
    if len(history['passed']) != report.get('passed'):
        raise RuntimeError('Focus baseline native pass count does not match its qualified identities.')
    for name, record in report.get('doctests', {}).items():
        value = record.get('evidence', {})
        spec = {'repo': report['repository'], 'package': name,
                'command': _test_command070(name, 'doc', feature_selection.get(name))}
        row = report.get('readouts', {}).get(record.get('guard_label'), {})
        if (qualify_doc055(spec, value, row)
                and _successful_row058(report, record.get('guard_label'))):
            history['docs'][name] = 0
    for record in report.get('custom_harnesses', []):
        if record.get('outcome') != 'passed' or not _successful_row058(report, record.get('guard_label')):
            continue
        package = record['package']
        artifacts = compilations.get(package, {}).get('evidence', {}).get('artifacts', {})
        matches = [key for key, artifact in artifacts.items()
                   if artifact.get('target', {}).get('name') == record.get('target')
                   and artifact.get('target', {}).get('harness') is False]
        if len(matches) != 1:
            raise RuntimeError('Focus baseline custom harness has an ambiguous Cargo target.')
        history['customs'][(package, matches[0])] = 0
    return history, report


def load_focus058(repo, path):
    base = support055()
    path = Path(os.path.abspath(Path(path).expanduser()))
    _, common = base.repository_dirs033(repo)
    if '..' in path.parts or common / 'codex-repairs' not in path.parents:
        raise RuntimeError('Focus receipt must be retained inside this repository repair metadata.')
    fingerprint, raw = base.read_file033(path, 32 * MIB055)
    value = _json055(raw)
    if canonical055(value) != raw:
        raise RuntimeError('Focus receipt is not canonical generated JSON.')
    report = value.get('verification', value) if isinstance(value, dict) else None
    if not isinstance(report, dict) or report.get('repository') != str(repo):
        raise RuntimeError('Focus receipt belongs to a different working tree.')
    source = {'path': str(path), 'fingerprint': fingerprint, 'candidate_head': value.get('candidate_head', report.get('current_head')),
              'run_directory': report.get('run_directory')}
    history, previous = history_from_report058(value, source)
    return history, {'path': str(path), 'fingerprint': fingerprint, 'candidate_head': source['candidate_head'],
                     'run_directory': previous.get('run_directory'), 'previous_outcome': previous.get('outcome'),
                     'retained_native_passes': len(history['passed']), 'retained_doctest_commands': len(history['docs']),
                     'retained_custom_commands': len(history['customs']),
                     'feature_selection070': _feature_selection070(previous.get('feature_selection070')),
                     'interpretation': 'Historical results under their recorded source and controls; not fresh executions or a current full-suite claim.'}


def package_pending058(package, history):
    name = package['name']
    if name not in history['compiled']:
        return True
    for target in package['targets']:
        if not target['native_eligible']:
            continue
        key = target['key']
        if target.get('excluded_by_default_features') and name + '::' + key not in history['lists']:
            continue
        if target.get('harness') is False:
            if (name, key) not in history['customs']:
                return True
            continue
        record = history['lists'].get(name + '::' + key)
        if record is None or any((name, key, case) not in history['passed'] for case in record['all']):
            return True
    return any(target.get('doctest_eligible') for target in package['targets']) and name not in history['docs']


def continuation_state058(report, previous):
    """Carry accumulated history forward without converting it into fresh proof."""
    history = previous if previous is not None else empty_history058()
    compiled = set(history['compiled'])
    lists = dict(history['lists'])
    passed, docs, customs = dict(history['passed']), dict(history['docs']), dict(history['customs'])
    sources = list(history['sources'])
    index = len(sources)
    sources.append({'kind': 'current_run', 'run_directory': report.get('run_directory'),
                    'candidate_head': report.get('current_head')})
    for name, record in report.get('compilations', {}).items():
        if (record.get('qualified058') is True
                and record.get('evidence', {}).get('outcome') in {'compiled', 'compiled_no_native_targets'}
                and _successful_row058(report, record.get('guard_label'))):
            compiled.add(name)
        else:
            compiled.discard(name)
    lists.update(report.get('test_lists', {}))
    for case in report.get('cases', []):
        if case['assessment']['outcome'] == 'passed' and _successful_row058(report, case.get('guard_label')):
            passed[(case['package'], case['target_key'], case['name'])] = index
    for name, record in report.get('doctests', {}).items():
        if (record.get('qualified058') is True and record.get('evidence', {}).get('outcome') == 'passed'
                and _successful_row058(report, record.get('guard_label'))):
            docs[name] = index
    for record in report.get('custom_harnesses', []):
        if record.get('outcome') == 'passed' and _successful_row058(report, record.get('guard_label')):
            customs[(record['package'], record['target_key'])] = index
    return {'schema': 'bitcoinrs-subsystem-continuation058-v1', 'repository': report['repository'],
            'workspace_packages': report.get('workspace_packages', history['workspace_packages']),
            'compiled_packages': sorted(compiled), 'target_lists': lists,
            'passed_native': [list(key) + [source] for key, source in sorted(passed.items())],
            'passed_doctests': [[key, source] for key, source in sorted(docs.items())],
            'passed_customs': [list(key) + [source] for key, source in sorted(customs.items())],
            'sources': sources,
            'interpretation': 'Historical and fresh entries retain separate source references; this state is a continuation inventory, not a new full-suite qualification.'}



def verify055(repo, *, run_directory=None, target_directory=None, already_locked=False,
              before_phase=None, packages=None, focus_report_path=None, features070=None):
    feature_selection = _feature_selection070(features070)
    if feature_selection and (packages is None or set(packages) != set(feature_selection)):
        raise RuntimeError('Named Snappy verification requires exactly its explicit selected packages.')
    base = support055()
    report = {'workflow': WORKFLOW055, 'outcome': 'failed', 'phase': 'preflight',
              'repository': str(repo), 'validation_reused': False, 'source_writes_started': False,
              'run_mode058': 'failed-and-unexecuted' if focus_report_path is not None else 'full',
              'retained_case_skips': [], 'retained_custom_skips': [],
              'refs_updated': False, 'full_coverage_claimed': False, 'feature_matrix_claimed': False,
              'host_default_features_only': not bool(feature_selection),
              'feature_selection070': feature_selection, 'builds_started': False, 'native_tests_started': False,
              'packages': [], 'cases': [], 'custom_harnesses': [], 'compilations': {},
              'doctests': {}, 'readouts': {}, 'snapshots': [], 'failures': [], 'failed_source': {},
              'inputs_preserved': False, 'expected_test_count': 0, 'passed': 0,
              'scope': 'All discovered bitcoinleveldb-*, bitcoinleveldbt-* and bitcoin-leveldb workspace members; '
                       'host/default-feature native tests, ignored tests explicitly executed, custom harness commands, '
                       'and doctests. Benchmark targets and benchmark entries are reported but excluded. '
                       'All explicitly classified original stress cases run last. No retries or budget increases.',
              'explicit_stress_cases': [dict(package=p, name=n, marker=m) for (p, n), m in EXPLICIT_STRESS057.items()],
              'assessment_parser': 'exact-libtest-should-panic057',
              'doctest_envelope': '2 GiB RAM, one CPU, no swap, 600 seconds, 512 MiB private test data; includes rustdoc compilation.'}
    if feature_selection:
        report['scope'] = ('Explicit named Snappy feature verification for the selected packages only; '
            'native and doctest execution retain the existing finite controls. No all-features or full-subsystem matrix claim.')
    locks = []
    baseline = None
    baseline_sha = None
    run = None
    guard = None
    failed_packages = set()
    deferred = []
    observed_known = set()
    focus_history = None
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        _, common = base.repository_dirs033(repo)
        own = common / 'codex-repairs' / WORKFLOW055
        env = base.checked_environment033()
        if already_locked:
            if run_directory is None or target_directory is None:
                raise RuntimeError('Installer verification requires explicit run and cache paths.')
        else:
            if run_directory is not None or target_directory is not None:
                raise RuntimeError('Private run/cache overrides are installer-only.')
            locks = lock_roots055(base, common, own)
            stamp = dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8]
            runs = own / 'runs'
            runs.mkdir(mode=0o700, exist_ok=True)
            run_directory = runs / stamp
            warm = common / 'codex-repairs/leveldb-final-crates-runner033/cargo-target'
            target_directory = warm if warm.is_dir() else own / 'cargo-target'
        run, target = Path(run_directory), Path(target_directory)
        if not run.is_absolute() or not target.is_absolute() or '..' in run.parts or '..' in target.parts:
            raise RuntimeError('Run/cache paths must be ordinary absolute paths.')
        run.mkdir(mode=0o700, parents=True, exist_ok=True)
        target.mkdir(mode=0o700, parents=True, exist_ok=True)
        for path in (run, target):
            fd = base.ordinary_dir(path)
            os.close(fd)
        if common / 'codex-repairs' not in run.parents or common / 'codex-repairs' not in target.parents:
            raise RuntimeError('Run/cache must have explicit repository repair-metadata ownership.')
        if any((run / leaf).exists() for leaf in ('RUNNING.json', 'SUBSYSTEM_RETURN.json')):
            raise RuntimeError('Run directory is occupied; earlier evidence was retained.')
        report.update(repository=str(repo), run_directory=str(run), target_directory=str(target))
        report['current_head'] = base.git033(repo, ('rev-parse', 'HEAD')).decode('utf-8').strip()
        json055(run / 'RUNNING.json', {'workflow': WORKFLOW055, 'repository': str(repo), 'phase': 'reserved'})
        gm = base.load_guard033(repo)
        guard = gm.Guard(run / 'guard', SCRIPT055, env)
        nix = shutil.which('nix-shell', path=env.get('PATH'))
        if nix is None:
            raise RuntimeError('Offline nix-shell is unavailable; no unrestricted fallback was attempted.')
        nix = os.path.abspath(nix)

        def nix_command(arguments):
            return [nix, '--option', 'max-jobs', '0', '--option', 'builders', '', '--option',
                    'substitute', 'false', str(repo / 'shell.nix'), '--run',
                    shlex.join([sys.executable, '-I', '-S', str(SCRIPT055), *map(str, arguments)])]

        def phase(label, command, *, kind='case', extra=None, data=None, artifact=None, require=True):
            report['phase'] = label
            report['inputs_preserved'] = False
            print('Protected subsystem phase: ' + label, flush=True)
            if before_phase:
                before_phase(label)
            if shutil.disk_usage(run).free < 2 * 1024 * MIB055:
                raise RuntimeError('The 2 GiB free-space floor prevents the next phase.')
            actual_env = dict(extra or {})
            if data is None and kind in {'case', 'stress'}:
                data = run / ('case-data-phase-' + hashlib.sha256(label.encode()).hexdigest()[:20])
            if data is not None:
                data.mkdir(mode=0o700, parents=True, exist_ok=False)
                actual_env.update({key: str(data) for key in ('TEST_TMPDIR', 'TMPDIR', 'TMP', 'TEMP')})
            row = guard.run(label, list(command), repo, actual_env, kind=kind, tmpdir=data)
            compact = {key: row.get(key) for key in ('kind', 'outcome', 'verified', 'exit_code', 'command',
                'cwd', 'data_directory', 'log', 'elapsed_seconds', 'output_bytes', 'data_bytes', 'data_entries')}
            compact['result_file'] = str(Path(row['log']).with_name('result.json')) if row.get('log') else None
            if row.get('outcome') != 'exited' or row.get('exit_code') != 0:
                compact['tail'] = row.get('tail', '')[-32768:]
                compact['error'] = row.get('error')
            report['readouts'][label] = compact
            if row.get('verified'):
                proof, text = binding055(base, gm, run, row, command, repo, actual_env, kind, artifact)
                report.setdefault('execution_bindings', {})[label] = proof
            else:
                raise RuntimeError('Finite controls could not be verified for ' + label)
            if row.get('outcome') in {'disk_free_limit', 'controls_unavailable', 'guard_error',
                                       'restricted_nix_setting_ignored', 'interrupted'}:
                raise RuntimeError('Subsystem execution stopped: ' + row['outcome'])
            if require and (row.get('outcome') != 'exited' or row.get('exit_code') != 0):
                raise RuntimeError('Guarded phase did not succeed: ' + label + ': ' + row.get('outcome', 'unknown'))
            return row, text

        def capture(label):
            nonlocal baseline, baseline_sha
            report['inputs_preserved'] = False
            output = run / ('inputs-' + format(len(report['snapshots']), '04d') + '.json')
            command = [sys.executable, '-I', '-S', str(SCRIPT055), '--snapshot055', str(repo),
                       '-' if baseline is None else str(baseline), str(output)]
            phase('inputs-' + label, command, kind='source')
            value, fp = read_json055(output)
            if baseline is None:
                baseline = output
                baseline_sha = hashlib.sha256(canonical055(value)).hexdigest()
                same = True
                report['inputs_before'] = {'path': str(output), 'fingerprint': fp, 'sha256': baseline_sha}
            else:
                same = (value.get('inputs_preserved') is True and value.get('canonical_sha256') == baseline_sha
                        and value.get('baseline') == str(baseline)
                        and value.get('baseline_fingerprint') == report['inputs_before']['fingerprint']
                        and base.read_file033(baseline, 16 * MIB055, False)[0] == report['inputs_before']['fingerprint'])
            report['snapshots'].append({'path': str(output), 'fingerprint': fp, 'matches_initial': same})
            report['inputs_preserved'] = same
            if not same:
                raise RuntimeError('Working sources, runner code or Git metadata changed: ' + label)

        def failure(package, stage, error):
            failed_packages.add(package['name'])
            report['failures'].append({'package': package['name'], 'stage': stage, 'error': str(error)[:2048]})

        def execute_case(package, artifact, name, ignored, known=None):
            index = len(report['cases'])
            suffix = known['suffix'] if known else 'test-' + format(index, '06d')
            data = run / ('case-data-' + suffix)
            if known:
                command = list(base.case_command033(artifact['path'], known))
                extra = dict(base.case_environment033(known, data))
                kind = 'stress' if known['profile'] == 'full_original' else 'case'
                marker = known['marker']
            else:
                command = [artifact['path'], '--exact', name]
                if ignored:
                    command.append('--ignored')
                command += ['--nocapture', '--test-threads=1']
                extra = {'RUST_LOG': 'off', 'RUST_BACKTRACE': '0', 'TEST_RANDOM_SEED': '301'}
                explicit = EXPLICIT_STRESS057.get((package['name'], name))
                kind = 'stress' if ignored or (package['name'], name) in EXPLICIT_STRESS057 else 'case'
                marker = explicit
            row, text = phase('case-' + format(index, '06d'), command, extra=extra, data=data,
                              kind=kind, artifact=artifact, require=False)
            report['native_tests_started'] = True
            assessment = assess_case057(row, text, name, marker)
            record = {'package': package['name'], 'target': artifact['target']['name'], 'target_key': artifact['target']['key'], 'name': name,
                      'ignored_explicitly_run': ignored, 'kind': kind, 'assessment': assessment,
                      'guard_label': row['label'], 'artifact_fingerprint': artifact['fingerprint']}
            if known:
                record['repaired_case'] = {'crate': known['crate'], 'suffix': known['suffix'], 'profile': known['profile']}
            report['cases'].append(record)
            if assessment['outcome'] != 'passed':
                failure(package, 'test:' + name, assessment['error'])

        if focus_report_path is not None:
            focus_history, focus_reference = load_focus058(repo, focus_report_path)
            if focus_reference['feature_selection070'] != feature_selection:
                raise RuntimeError('Focus receipt uses a different default/Snappy feature selection.')
            report['focus_baseline058'] = focus_reference
            report['scope'] = ('Focused fresh verification of failed and unexecuted checks from the retained report; '
                               'historical passes are not executed again or counted as fresh. Source and resource controls remain required.')
        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('SUBSYSTEM055_PREFLIGHT_OK')"], kind='probe')
        capture('initial')
        metadata_file = run / 'cargo-metadata.json'
        phase('workspace-metadata', nix_command(['--metadata055', repo, metadata_file]), kind='build',
              extra={'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0'})
        metadata, meta_fp = read_json055(metadata_file)
        report['metadata_context'] = metadata_context055(metadata, repo)
        workspace_packages = discover055(metadata, repo)
        report['workspace_packages'] = workspace_packages
        report['workspace_package_count058'] = len(workspace_packages)
        chosen = select_packages055(workspace_packages, packages)
        if focus_history is not None:
            chosen = [package for package in chosen if package_pending058(package, focus_history)]
        report['selected_package_names058'] = [package['name'] for package in chosen]
        report['metadata_fingerprint'] = meta_fp
        report['packages'] = chosen
        expected_known = {(base.TARGETS033[x['crate']]['package'], x['target'], x['name'])
                          for x in base.CASES033 if base.TARGETS033[x['crate']]['package'] in {p['name'] for p in chosen}}
        report['selected_package_count'] = len(chosen)
        for number, package in enumerate(chosen):
            name = package['name']
            capture('before-package-' + format(number, '03d'))
            command = _test_command070(name, 'compile', feature_selection.get(name))
            spec_path, evidence_path = run / ('compile-' + name + '.spec.json'), run / ('compile-' + name + '.json')
            spec = {'repo': str(repo), 'run': str(run), 'target': str(target), 'package': package,
                    'evidence_path': str(evidence_path), 'command': command}
            json055(spec_path, spec, 2 * MIB055)
            row, _ = phase('compile-' + name, nix_command(['--observe055', spec_path]), kind='build',
                           extra={'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0'}, require=False)
            report['builds_started'] = True
            evidence = None
            if evidence_path.exists():
                evidence, evidence_fp = read_json055(evidence_path, 2 * MIB055)
                report['compilations'][name] = {'evidence_path': str(evidence_path), 'fingerprint': evidence_fp,
                                               'evidence': evidence, 'guard_label': row['label']}
            artifacts = qualify_observer055(spec, evidence, row)
            if artifacts is None:
                failure(package, 'compile', 'Compilation or its package/target/executable binding failed; see retained compiler evidence.')
                capture('after-package-' + format(number, '03d'))
                continue
            report['compilations'][name]['qualified058'] = True
            for key, artifact in sorted(artifacts.items()):
                info = artifact['target']
                if info.get('harness') is False:
                    if focus_history is not None and (name, key) in focus_history['customs']:
                        report['retained_custom_skips'].append([name, key])
                        continue
                    data = run / ('case-data-custom-' + format(len(report['custom_harnesses']), '05d'))
                    item, _ = phase('custom-' + name + '-' + info['name'], [artifact['path']], kind='stress',
                                    extra={'RUST_LOG': 'off', 'RUST_BACKTRACE': '0', 'TEST_RANDOM_SEED': '301'},
                                    data=data, artifact=artifact, require=False)
                    okay = item.get('outcome') == 'exited' and item.get('exit_code') == 0
                    report['custom_harnesses'].append({'package': name, 'target': info['name'], 'target_key': key,
                        'outcome': 'passed' if okay else 'failed', 'internal_case_count': None,
                        'guard_label': item['label'], 'coverage': 'custom command exit status only'})
                    report['native_tests_started'] = True
                    if not okay:
                        failure(package, 'custom-harness:' + info['name'], 'Custom harness did not exit successfully.')
                    continue
                lists = {}
                benchmark_lists = {}
                list_failed = False
                for selection, args in (('all', ['--list']), ('ignored', ['--list', '--ignored'])):
                    label = 'list-' + format(number, '03d') + '-' + hashlib.sha256(key.encode()).hexdigest()[:10] + '-' + selection
                    list_row, text = phase(label, [artifact['path'], *args], artifact=artifact, require=False)
                    if list_row.get('outcome') != 'exited' or list_row.get('exit_code') != 0:
                        failure(package, 'test-list:' + info['name'], 'The guarded test listing did not exit successfully.')
                        list_failed = True
                        break
                    try:
                        lists[selection] = parse_list055(text)
                        benchmark_lists[selection] = parse_benchmarks055(text)
                    except RuntimeError as error:
                        failure(package, 'test-list:' + info['name'], error)
                        list_failed = True
                        break
                if list_failed:
                    continue
                if not set(lists['ignored']).issubset(lists['all']):
                    failure(package, 'test-list:' + info['name'], 'Ignored tests are absent from the complete listing.')
                    continue
                report.setdefault('test_lists', {})[name + '::' + key] = lists
                report.setdefault('benchmark_exclusions', {})[name + '::' + key] = benchmark_lists
                for case_name in lists['all']:
                    known = known_case055(base, name, info['name'], case_name)
                    ignored = case_name in lists['ignored']
                    if known:
                        if ignored != known['ignored']:
                            raise RuntimeError('A repaired test changed its explicit ignored/stress classification.')
                        observed_known.add((name, info['name'], case_name))
                    if focus_history is not None and (name, key, case_name) in focus_history['passed']:
                        report['retained_case_skips'].append([name, key, case_name])
                        continue
                    report['expected_test_count'] += 1
                    if report['expected_test_count'] > 20000:
                        raise RuntimeError('The 20000-case run allowance was reached.')
                    if (known and known['profile'] == 'full_original') or (name, case_name) in EXPLICIT_STRESS057:
                        deferred.append((package, artifact, case_name, ignored, known))
                    else:
                        execute_case(package, artifact, case_name, ignored, known)
            if focus_history is not None and name in focus_history['docs']:
                report['doctests'][name] = {'outcome': 'retained_baseline_pass', 'fresh_execution': False}
            elif any(t.get('doctest_eligible') for t in package['targets']):
                doc_path = run / ('doc-' + name + '.spec.json')
                doc_evidence = run / ('doc-' + name + '.json')
                doc_command = _test_command070(name, 'doc', feature_selection.get(name))
                doc_spec = {'repo': str(repo), 'package': name, 'command': doc_command,
                            'evidence_path': str(doc_evidence)}
                json055(doc_path, doc_spec)
                doc_data = run / ('case-data-doc-' + format(number, '03d'))
                doc_row, _ = phase('doc-' + name, nix_command(['--doc055', doc_path]), kind='stress',
                    extra={'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0',
                           'RUST_LOG': 'off', 'TEST_RANDOM_SEED': '301'}, data=doc_data, require=False)
                if doc_evidence.exists():
                    doc_value, doc_fp = read_json055(doc_evidence, 2 * MIB055)
                    report['doctests'][name] = {'evidence': doc_value, 'fingerprint': doc_fp, 'guard_label': doc_row['label']}
                    okay = qualify_doc055(doc_spec, doc_value, doc_row)
                    report['doctests'][name]['qualified058'] = okay
                else:
                    okay = False
                if not okay:
                    failure(package, 'doctests', 'Doctests or their bounded evidence did not pass.')
            else:
                report['doctests'][name] = {'outcome': 'not_applicable', 'reason': 'No default doctest-enabled library target.'}
            capture('after-package-' + format(number, '03d'))
            report['completed_package_count'] = number + 1
            json055(run / 'PROGRESS.json', report)
        missing = sorted(expected_known - observed_known)
        if missing:
            report['failures'].append({'stage': 'repaired-test-inventory', 'missing': missing})
        capture('before-original-stress')
        for package, artifact, name, ignored, known in deferred:
            execute_case(package, artifact, name, ignored, known)
        capture('final')
        report['phase'] = 'complete'
        report['passed'] = sum(x['assessment']['outcome'] == 'passed' for x in report['cases'])
        report['test_count'] = len(report['cases'])
        report['doctests_passed'] = sum(x.get('evidence', {}).get('passed', 0) for x in report['doctests'].values())
        report['outcome'] = 'passed' if (not report['failures'] and report['inputs_preserved']
            and report['test_count'] == report['expected_test_count']) else 'failed'
    except BaseException as error:
        report['error'] = type(error).__name__ + ': ' + str(error)[:2048]
        report['outcome'] = 'incomplete' if not isinstance(error, KeyboardInterrupt) else 'interrupted'
    finally:
        if run is not None:
            # Failure collection reads only source in packages already discovered.
            allowance = 4 * MIB055
            for package in report['packages']:
                if package['name'] not in failed_packages or allowance <= 0:
                    continue
                try:
                    collected = failure_sources055(base, Path(report['repository']), package, min(MIB055, allowance))
                    report['failed_source'][package['name']] = collected
                    allowance -= collected['bytes']
                except Exception as error:
                    report['failed_source'][package['name']] = {'error': str(error)[:512]}
            report['passed'] = sum(x['assessment']['outcome'] == 'passed' for x in report['cases'])
            report['test_count'] = len(report['cases'])
            report['package_results'] = {}
            for package in report['packages']:
                name = package['name']
                compilation = report['compilations'].get(name, {}).get('evidence', {})
                lists = [value for key, value in report.get('test_lists', {}).items() if key.startswith(name + '::')]
                cases = [case for case in report['cases'] if case['package'] == name]
                failed = [entry for entry in report['failures'] if entry.get('package') == name]
                report['package_results'][name] = {
                    'compilation': compilation.get('outcome', 'not_completed'),
                    'discovered_native_tests': sum(len(value['all']) for value in lists),
                    'native_tests_run': len(cases),
                    'native_tests_passed': sum(case['assessment']['outcome'] == 'passed' for case in cases),
                    'custom_harness_commands': sum(x['package'] == name for x in report['custom_harnesses']),
                    'doctests': report['doctests'].get(name, {}).get('evidence', report['doctests'].get(name, {})),
                    'feature_exclusions': compilation.get('excluded_targets', []),
                    'non_test_targets': [{'key': item['key'], 'kind': item['kind'], 'test': item['test']}
                                         for item in package['targets'] if not item['native_eligible']],
                    'failure_count': len(failed),
                    'zero_native_tests_observed': bool(lists) and not any(value['all'] for value in lists)}
            report['continuation_state058'] = continuation_state058(report, focus_history)
            report['historical_native_pass_count058'] = len(focus_history['passed']) if focus_history is not None else 0
            report['fresh_native_pass_count058'] = report['passed']
            report['focused_verification_passed058'] = report['run_mode058'] == 'failed-and-unexecuted' and report['outcome'] == 'passed'
            report['return_file'] = str(run / 'SUBSYSTEM_RETURN.json')
            json055(run / 'SUBSYSTEM_RETURN.json', report)
        for fd in reversed(locks):
            os.close(fd)
    return report


def parse_list055(text):
    names = []
    for line in text.splitlines():
        if line.endswith(': test'):
            name = line[:-6]
            if not name or re.search(r'\s', name) or name in names:
                raise RuntimeError('Executable test listing has an invalid or duplicate test name.')
            names.append(name)
    summaries = re.findall(r'(?m)^(\d+) tests?, (\d+) benchmarks?$', text)
    if len(summaries) != 1 or int(summaries[0][0]) != len(names):
        raise RuntimeError('Executable listing lacks one complete matching test-count summary.')
    return names


def parse_benchmarks055(text):
    names = [line[:-11] for line in text.splitlines() if line.endswith(': benchmark')]
    summaries = re.findall(r'(?m)^(\d+) tests?, (\d+) benchmarks?$', text)
    if len(summaries) != 1 or int(summaries[0][1]) != len(names) or len(names) != len(set(names)):
        raise RuntimeError('Executable benchmark listing lacks a complete matching count.')
    return {'names': names, 'count': len(names), 'scope': 'reported, not executed as benchmarks'}


def main055(argv=None):
    arguments = list(sys.argv[1:] if argv is None else argv)
    if arguments[:1] == ['--guard-child'] and len(arguments) == 2:
        return support055().load_guard033().guard_child(arguments[1])
    if arguments[:1] == ['--snapshot055'] and len(arguments) == 4:
        return snapshot_worker055(*arguments[1:])
    if arguments[:1] == ['--metadata055'] and len(arguments) == 3:
        return metadata_worker055(*arguments[1:])
    if arguments[:1] == ['--observe055'] and len(arguments) == 2:
        return observe055(arguments[1])
    if arguments[:1] == ['--doc055'] and len(arguments) == 2:
        return doc_worker055(arguments[1])
    parser = argparse.ArgumentParser(description='Run discovered LevelDB workspace crates under finite resource controls.')
    parser.add_argument('--repo', default=str(SCRIPT055.parents[2]))
    parser.add_argument('--crate', action='append', dest='packages', help='An exact discovered package name; repeat to select several.')
    parser.add_argument('--failed-from', dest='focus_report_path', help='A retained subsystem receipt: execute only failed and unexecuted checks, preserving historical passes separately.')
    args = parser.parse_args(arguments)
    report = verify055(args.repo, packages=args.packages, focus_report_path=args.focus_report_path)
    print('BEGIN_SUBSYSTEM_RETURN')
    print(json.dumps(report, sort_keys=True, indent=2))
    print('END_SUBSYSTEM_RETURN')
    return 0 if report['outcome'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main055())
