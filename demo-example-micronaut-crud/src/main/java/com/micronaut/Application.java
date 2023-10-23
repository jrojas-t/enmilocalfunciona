package com.micronaut;

import com.micronaut.model.Client;
import com.micronaut.repository.ClientRepository;
import io.micronaut.context.event.StartupEvent;
import io.micronaut.runtime.Micronaut;
import io.micronaut.runtime.event.annotation.EventListener;
import jakarta.inject.Singleton;
import jakarta.transaction.Transactional;

import java.util.Arrays;

@Singleton
public class Application {

    private final ClientRepository repository;

    public Application(ClientRepository repository){
        this.repository = repository;
    }

    public static void main(String[] args) {
        Micronaut.run(Application.class, args);
    }

    @EventListener
    @Transactional
    public void init(StartupEvent startupEvent) {
        repository.saveAll(Arrays.asList(
                new Client( "Jhonatan", 33, "Av. Diagonal 735", 1.500),
                new Client( "Milton", 50, "Av. Desierto 500", 3.500),
                new Client( "Charles", 52, "Av. Juan Carlos I 10", 7.000)));
    }
}
