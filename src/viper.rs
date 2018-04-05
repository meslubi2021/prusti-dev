// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use jni::*;
use std::fs;
use verification_context::*;
use error_chain::ChainedError;
use std::env;

pub struct Viper {
    jvm: JavaVM,
}

impl Default for Viper {
    fn default() -> Self {
        Self::new()
    }
}

impl Viper {
    pub fn new() -> Self {
        let viper_home = env::var("VIPER_HOME").unwrap_or_else(|_| "/usr/lib/viper/".to_string());

        debug!("Using Viper home: '{}'", &viper_home);

        let jar_paths: Vec<String> = fs::read_dir(viper_home)
            .unwrap()
            .map(|x| x.unwrap().path().to_str().unwrap().to_string())
            .filter(|x| !x.contains("carbon"))
            .collect();

        debug!("Java classpath: {}", jar_paths.clone().join(":"));

        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option(&format!("-Djava.class.path={}", jar_paths.join(":")))
            .option("-Xdebug")
            .option("-Xmx2048m")
            .option("-Xss16m") // stack
            //.option("-verbose:gc")
            //.option("-Xcheck:jni")
            //.option("-XX:+CheckJNICalls")
            //.option("-Djava.security.debug=all")
            //.option("-verbose:jni")
            //.option("-XX:+TraceJNICalls")
            .build()
            .unwrap_or_else(|e| {
                panic!(format!("{}", e.display_chain().to_string()));
            });

        let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| {
            panic!(format!("{}", e.display_chain().to_string()));
        });

        let this = Viper { jvm };

        {
            // Initialize the logging context
            // See:
            // - https://bitbucket.org/viperproject/silicon/issues/315/exception-while-building-silicon-instances
            // - https://stackoverflow.com/a/16013093/2491528
            let ver_context = this.new_verification_context();
            ver_context.new_verifier();
        }

        this
    }

    pub fn new_verification_context(&self) -> VerificationContext {
        let env_guard = self.jvm
            .attach_current_thread()
            .expect("failed to attach jvm thread");

        VerificationContext::new(env_guard)
    }
}
