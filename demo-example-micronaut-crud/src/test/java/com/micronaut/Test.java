package com.micronaut;

import io.micronaut.runtime.EmbeddedApplication;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import org.junit.jupiter.api.Assertions;

import jakarta.inject.Inject;

@MicronautTest
class Test {

    @Inject
    EmbeddedApplication<?> application;

    @org.junit.jupiter.api.Test
    void testItWorks() {
        Assertions.assertTrue(application.isRunning());
    }

}
