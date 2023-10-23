package com.micronaut.repository;

import com.micronaut.model.Client;
import io.micronaut.data.jdbc.annotation.JdbcRepository;
import io.micronaut.data.model.query.builder.sql.Dialect;
import io.micronaut.data.repository.CrudRepository;

import java.util.List;

@JdbcRepository(dialect = Dialect.H2)
public interface ClientRepository extends CrudRepository<Client, Long> {

    @Override
    public List<Client> findAll();

}
