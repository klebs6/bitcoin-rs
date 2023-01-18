crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/status_test.cc]

#[test] fn status_move_constructor() {
    todo!();
    /*
    
      {
        crate::Status ok = crate::Status::OK();
        crate::Status ok2 = std::move(ok);

        ASSERT_TRUE(ok2.ok());
      }

      {
        crate::Status status = crate::Status::NotFound("custom NotFound status message");
        crate::Status status2 = std::move(status);

        ASSERT_TRUE(status2.IsNotFound());
        ASSERT_EQ("NotFound: custom NotFound status message", status2.ToString());
      }

      {
        crate::Status self_moved = crate::Status::IOError("custom IOError status message");

        // Needed to bypass compiler warning about explicit move-assignment.
        crate::Status& self_moved_reference = self_moved;
        self_moved_reference = std::move(self_moved);
      }

    */
}

pub fn teststatus_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
