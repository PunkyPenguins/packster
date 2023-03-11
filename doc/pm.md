commands :
- package pack - create a package
given a package source directory path ( fail if path does not exists or is not a directory )
given a destination path
reads packster-manifest.toml ( fail if file not exists or parsing fail )
create local location and merge pack-location IF ANY
execute "any" and pack handlers with an executor ( powershell by example ) IF ANY
create an archive of the directory in a tmp path
compute a checksum
move to path & rename archive with checksum in filename
delete local location

- package install-file
given package file path ( fail if path does not exists or is not a directory )
given a profil id
given arbitrary json-compliant key/value parameters
verifies file checksum
create packster-deployment.json in temporary directory
compute deployment hash from parameter file hash + package checksum
check there is no other deployment with the same hash ( or exit 0 ) ( or --force then removes the directory )
unpacks file in { location deployment path } / { deployment hash }
move packster-deployment.json to { location deployment path } / { deployment hash }
reads packster-location.json
reads packster-manifest.toml
verify that no other package provides the same resources ( or fail and rollback )
merge install-bundle to given location bundle if any IF ANY
update packages and resources in lockfile in { location deployment path } / packster-location.json


- bundle install
- bundle delete
- location enable
- upgrade
- uninstall
- pin
- locate
- location create
- location setup
    create in a default User Directory

do not need to be installed, just a single binary stuff, writing config in AppData or home directory or current directory as fallback
multiple installed version of the same package by design
a default distribution for a given package
user isolation
manage access rights
semver constraints / range ( adapter flexible to other versionning systems )
service / port isolation ?
integrable with artifactory
multiplatform
multi-languages
create pre-install bomb
supports bundling
package parameters
signing check
integrity check
secfirst
handle build dependencies
testing
cleanup
os constraint ( windows / linux / mac + version )
hardware constraints ( bitness )
supports any scripting language
support content based cache
PATH mgmt / location switching support ( location sets with shims / symlinks)
multi-sources
pinning
CI friendly
licensing + constraints
multi-param packages
pattern locator
guard for unique system resources ( env keys by example : if the same key is used or write by differente package, block it )
always keep consistent, predictable system state

deploy everyting :
- libraries
- cli
- guis
- binaries

hooks :
- packaging
- install
- upgrade
- uninstall
- always ( execute with an argument )


source :
    - MAY contains packages
    - MUST have searching, listing and retrieving capabilites
package :
    - MUST have a filename as : {id}-{semver}-{hash}.packster
    - MUST be a archive file
    - MUST have a unique identifier
    - MUST have a strict semver or "latest"
    - MUST have an integrity hash
    - MAY have a location to merge with the one their installed in
    - MAY have handlers for pack, install, upgrade, uninstall, any
    - MAY have a locator
    - MAY define shims
    - MAY define what it provides ( environment variables keys, shims, shared directory, shared file, network port )
deployment :
    - specific instance of an package
    - MUST be installed to comply to one or many semver constraint within a given location
    - MUST be identified by an integrity hash
bundle :
    - MAY have a set of semver constraints
    - MAY contains parameters for packages
location :
    locations could be stored in ENVIRONMENT for discovery, with just a path to the location deployment
    - installed bundle
    - can extend its bundle by merging other bundles définitions inside
    - MUST have a id and a hash
    - MUST have a unique path where to store deployments
    - MUST have a lockfile
    - MIGHT be set as system ( can fail if there is resources conflicts )
    - MIGHT be loaded just for the shell
    - MAY have shims


https://github.com/yonaskolb/packster
https://medium.com/@sdboyer/so-you-want-to-write-a-package-manager-4ae9c17d9527



the smart thing here would be identification of a package.
a package ( all its versions ) wouldn't simply be identified by a name, but by (arbitrary ?) labels:
- target plaform ( bitness )
- target operating system
- compiler
- provider / vendor ?

if a user want to install from a name, some of those labels would be infered from system, and there is still conflict, the user would be asked for what he want to install
there is two main drawback to that approach :
- dependencies has to take in account that in dependency constraint definition
- all the labels has to be specified at package creation once and for all ( to ensure version contiguity )

what not package medias in the same way ?

// https://herbertograca.files.wordpress.com/2018/11/070-explicit-architecture-svg.png
// https://youtu.be/qBLtZN3p3FU?t=1117


## Target Features

Cargo semver
Multi-plateform package manager ( but potentially plateform-specific packages )

- Packaging
    - dependency manangement for packing needs
    - Simple file packing
    - script execution for packing
    - Wrapping deb / rpm / snap / appimage / chocolatey / winget packages
    - signing

- Install / Upgrade / Uninstall
    - dependency management at deployment
    - script execution at deployment
    - Integrity check
    - Multiple, isolated package location
    - Allow userspace package installation
    - Supports multiple versions of the same package
    - System-wide & local resources managements
        - environment variables
        - binary shims
        - network ports
        - shared directory
    - Default version selection
    - Default location selection